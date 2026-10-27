//! Live DNS Front Server: binds UDP and TCP listeners and forwards raw DNS
//! packets to loopback engine backends without altering them.
//!
//! Design guarantees (per docs/ARCHITECTURE.md):
//! - Packets are forwarded byte-for-byte; the router only reads the QNAME.
//! - Unmatched QNAMEs go to the optional default backend, otherwise a
//!   standards-compliant REFUSED (RCODE=5) response is generated.
//! - UDP responses relayed through the front socket; TCP uses RFC 1035
//!   two-byte length-prefixed messages.

use crate::{DnsRouter, RouteStats};
use luffy_core::CoreError;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};

const UDP_BUFFER_SIZE: usize = 4096;
const BACKEND_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct DnsFrontHandle {
    pub udp_addr: SocketAddr,
    pub tcp_addr: SocketAddr,
    pub stats: Arc<RouteStats>,
}

pub struct DnsFrontServer {
    bind: SocketAddr,
    router: Arc<RwLock<DnsRouter>>,
    stats: Arc<RouteStats>,
}

impl DnsFrontServer {
    /// Creates a front server bound to `bind` (usually `0.0.0.0:53`)
    /// dispatching according to the supplied [`DnsRouter`].
    pub fn new(bind: SocketAddr, router: DnsRouter) -> Self {
        Self {
            bind,
            router: Arc::new(RwLock::new(router)),
            stats: Arc::new(RouteStats::default()),
        }
    }

    /// Binds both listeners and spawns the two dispatch loops.
    /// Returns a handle with the effective bound addresses (port 0 supported
    /// for testing) without blocking.
    pub async fn start(self) -> Result<DnsFrontHandle, CoreError> {
        let udp = Arc::new(UdpSocket::bind(self.bind).await.map_err(sock_err)?);
        let tcp = TcpListener::bind(self.bind).await.map_err(sock_err)?;

        let udp_addr = udp.local_addr().map_err(sock_err)?;
        let tcp_addr = tcp.local_addr().map_err(sock_err)?;

        tracing::info!(%udp_addr, %tcp_addr, "Luffy DNS front router listening");

        tokio::spawn(Self::run_udp_loop(
            udp,
            self.router.clone(),
            self.stats.clone(),
        ));
        tokio::spawn(Self::run_tcp_loop(
            tcp,
            self.router.clone(),
            self.stats.clone(),
        ));

        Ok(DnsFrontHandle {
            udp_addr,
            tcp_addr,
            stats: self.stats,
        })
    }

    /// Resolves the backend for a raw query packet.
    /// `Err` when no route matches and no default backend is configured.
    fn resolve_backend(router: &RwLock<DnsRouter>, packet: &[u8]) -> Result<SocketAddr, CoreError> {
        let router = router.read().expect("router lock poisoned");
        match router.route_packet(packet) {
            Ok(addr) => Ok(addr),
            Err(CoreError::NotFound(_)) => match router.default_backend() {
                Some(default) => Ok(default),
                None => Err(CoreError::NotFound(
                    "no suffix route and no default backend configured".into(),
                )),
            },
            Err(e) => Err(e),
        }
    }

    async fn run_udp_loop(
        socket: Arc<UdpSocket>,
        router: Arc<RwLock<DnsRouter>>,
        stats: Arc<RouteStats>,
    ) {
        loop {
            let mut buf = vec![0u8; UDP_BUFFER_SIZE];
            let (n, peer) = match socket.recv_from(&mut buf).await {
                Ok(v) => v,
                Err(e) => {
                    stats.drops.fetch_add(1, Ordering::Relaxed);
                    tracing::warn!(error = %e, "UDP recv failed");
                    continue;
                }
            };
            stats.queries.fetch_add(1, Ordering::Relaxed);
            stats.bytes_in.fetch_add(n as u64, Ordering::Relaxed);

            let backend = Self::resolve_backend(&router, &buf[..n]);
            buf.truncate(n);
            let socket = socket.clone();

            tokio::spawn(async move {
                match backend {
                    Ok(backend) => {
                        if let Err(e) = forward_udp(&socket, &buf, backend, peer).await {
                            tracing::warn!(%backend, error = %e, "UDP forward failed");
                        }
                    }
                    Err(_) => {
                        let refused = build_refused_response(&buf);
                        let _ = socket.send_to(&refused, peer).await;
                    }
                }
            });
        }
    }

    async fn run_tcp_loop(
        listener: TcpListener,
        router: Arc<RwLock<DnsRouter>>,
        stats: Arc<RouteStats>,
    ) {
        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(v) => v,
                Err(e) => {
                    stats.drops.fetch_add(1, Ordering::Relaxed);
                    tracing::warn!(error = %e, "TCP accept failed");
                    continue;
                }
            };
            let router = router.clone();
            let stats = stats.clone();

            tokio::spawn(async move {
                if let Err(e) = serve_tcp_stream(stream, peer, router, stats).await {
                    tracing::warn!(%peer, error = %e, "TCP stream terminated");
                }
            });
        }
    }
}

async fn serve_tcp_stream(
    mut client: TcpStream,
    peer: SocketAddr,
    router: Arc<RwLock<DnsRouter>>,
    stats: Arc<RouteStats>,
) -> Result<(), CoreError> {
    let mut header = [0u8; 2];
    loop {
        // RFC 1035: TCP DNS messages are prefixed with a 16-bit length.
        let n = client.read(&mut header).await.map_err(io_err)?;

        if n == 0 {
            return Ok(()); // client closed connection
        }
        if n < 2 {
            return Err(CoreError::Validation("Truncated TCP length prefix".into()));
        }
        let msg_len = u16::from_be_bytes(header) as usize;
        let mut packet = vec![0u8; msg_len];
        client.read_exact(&mut packet).await.map_err(io_err)?;

        stats.queries.fetch_add(1, Ordering::Relaxed);
        stats
            .bytes_in
            .fetch_add((msg_len + 2) as u64, Ordering::Relaxed);

        let response = match DnsFrontServer::resolve_backend(&router, &packet) {
            Ok(backend) => {
                let relay = tokio::time::timeout(
                    BACKEND_TIMEOUT,
                    forward_tcp(&packet, backend),
                )
                .await
                .map_err(|_| CoreError::Internal("Backend TCP timeout".into()))??;
                stats.bytes_out.fetch_add(relay.len() as u64, Ordering::Relaxed);
                relay
            }
            Err(_) => build_refused_response(&packet),
        };

        let mut framed = Vec::with_capacity(response.len() + 2);
        framed.extend_from_slice(&(response.len() as u16).to_be_bytes());
        framed.extend_from_slice(&response);
        client.write_all(&framed).await.map_err(io_err)?;
        let _ = peer;
    }
}

/// Sends `packet` from a fresh ephemeral socket to `backend` and relays the
/// first answer back to the original `peer` through `front`. The query packet
/// is forwarded unmodified.
async fn forward_udp(
    front: &UdpSocket,
    packet: &[u8],
    backend: SocketAddr,
    peer: SocketAddr,
) -> Result<(), CoreError> {
    let outbound = UdpSocket::bind("0.0.0.0:0").await.map_err(sock_err)?;
    outbound.send_to(packet, backend).await.map_err(sock_err)?;

    let mut buf = vec![0u8; UDP_BUFFER_SIZE];
    let n = tokio::time::timeout(BACKEND_TIMEOUT, outbound.recv_from(&mut buf))
        .await
        .map_err(|_| CoreError::Internal("Backend UDP timeout".into()))?
        .map_err(sock_err)?
        .0;

    front.send_to(&buf[..n], peer).await.map_err(sock_err)?;
    Ok(())
}

/// Opens a TCP connection to the backend, sends one length-prefixed message
/// and returns the backend's length-prefixed response payload.
async fn forward_tcp(packet: &[u8], backend: SocketAddr) -> Result<Vec<u8>, CoreError> {
    let mut stream = tokio::time::timeout(BACKEND_TIMEOUT, TcpStream::connect(backend))
        .await
        .map_err(|_| CoreError::Internal("Backend TCP connect timeout".into()))?
        .map_err(sock_err)?;

    let mut framed = Vec::with_capacity(packet.len() + 2);
    framed.extend_from_slice(&(packet.len() as u16).to_be_bytes());
    framed.extend_from_slice(packet);
    stream.write_all(&framed).await.map_err(io_err)?;

    let mut header = [0u8; 2];
    stream.read_exact(&mut header).await.map_err(io_err)?;
    let msg_len = u16::from_be_bytes(header) as usize;
    let mut response = vec![0u8; msg_len];
    stream.read_exact(&mut response).await.map_err(io_err)?;
    Ok(response)
}

/// Builds a minimal REFUSED (RCODE=5) response echoing the original question,
/// as recommended by RFC 8906 for authoritative refusals.
pub fn build_refused_response(packet: &[u8]) -> Vec<u8> {
    if packet.len() < 12 {
        // Too short to interpret even the header; nothing sensible to reply.
        return Vec::new();
    }
    let mut resp = packet.to_vec();
    resp[2] |= 0x80; // QR = 1 (this is a response)
    resp[3] = (resp[3] & 0xF0) | 0x05; // RCODE = 5 (REFUSED)
    resp[6..12].fill(0); // ANCOUNT / NSCOUNT / ARCOUNT = 0
    resp
}

fn sock_err(e: std::io::Error) -> CoreError {
    CoreError::Internal(format!("Socket error: {e}"))
}

fn io_err(e: std::io::Error) -> CoreError {
    CoreError::Internal(format!("IO error: {e}"))
}
