//! End-to-end integration tests proving the live DNS front router forwards
//! real packets over UDP and TCP to real backend sockets.

use luffy_router::server::build_refused_response;
use luffy_router::{DnsFrontServer, DnsRouter};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, UdpSocket};

const TIMEOUT: Duration = Duration::from_secs(3);

/// Builds a real DNS A-record query packet for the given QNAME.
fn build_query(id: u16, qname: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&id.to_be_bytes());
    packet.extend_from_slice(&[0x01, 0x00]); // RD=1
    packet.extend_from_slice(&[0x00, 0x01]); // QDCOUNT=1
    packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    for label in qname.trim_end_matches('.').split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
    packet.extend_from_slice(&[0x00, 0x01]); // QTYPE = A
    packet.extend_from_slice(&[0x00, 0x01]); // QCLASS = IN
    packet
}

/// Spawns a fake UDP DNS backend that flips QR=1 on every received packet
/// and echoes it back, proving the packet traversed a real socket round-trip.
async fn spawn_udp_echo_backend() -> SocketAddr {
    let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let addr = socket.local_addr().unwrap();
    tokio::spawn(async move {
        let mut buf = [0u8; 4096];
        loop {
            let Ok((n, peer)) = socket.recv_from(&mut buf).await else { break };
            let mut reply = buf[..n].to_vec();
            reply[2] |= 0x80; // mark as response
            let _ = socket.send_to(&reply, peer).await;
        }
    });
    addr
}

#[tokio::test]
async fn test_udp_query_reaches_backend_and_answer_relayed() {
    let backend = spawn_udp_echo_backend().await;

    let mut router = DnsRouter::new();
    router.add_route("test.example", backend).unwrap();

    let handle = DnsFrontServer::new("127.0.0.1:0".parse().unwrap(), router)
        .start()
        .await
        .unwrap();

    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    client.connect(handle.udp_addr).await.unwrap();

    let query = build_query(0xBEEF, "app.test.example");
    client.send(&query).await.unwrap();

    let mut response = vec![0u8; 4096];
    let n = tokio::time::timeout(TIMEOUT, client.recv(&mut response))
        .await
        .expect("no UDP answer from front router (forwarding broken)")
        .unwrap();

    assert_eq!(response[0], 0xBE);
    assert_eq!(response[1], 0xEF); // DNS ID preserved
    assert!(response[2] & 0x80 != 0, "QR flag must be set by backend");
    assert_eq!(n, query.len());

    // Router stats must have counted the query
    assert_eq!(handle.stats.queries.load(std::sync::atomic::Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_tcp_query_forwarded_with_length_framing() {
    // Fake TCP DNS backend: reads length-prefixed messages, echoes with QR set.
    let backend_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let backend_addr = backend_listener.local_addr().unwrap();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = backend_listener.accept().await else { break };
            tokio::spawn(async move {
                loop {
                    let mut header = [0u8; 2];
                    if stream.read_exact(&mut header).await.is_err() {
                        break;
                    }
                    let len = u16::from_be_bytes(header) as usize;
                    let mut msg = vec![0u8; len];
                    if stream.read_exact(&mut msg).await.is_err() {
                        break;
                    }
                    msg[2] |= 0x80;
                    let mut framed = Vec::with_capacity(len + 2);
                    framed.extend_from_slice(&(len as u16).to_be_bytes());
                    framed.extend_from_slice(&msg);
                    let _ = stream.write_all(&framed).await;
                }
            });
        }
    });

    let mut router = DnsRouter::new();
    router.add_route("test.example", backend_addr).unwrap();

    let handle = DnsFrontServer::new("127.0.0.1:0".parse().unwrap(), router)
        .start()
        .await
        .unwrap();

    let mut client = tokio::time::timeout(TIMEOUT, tokio::net::TcpStream::connect(handle.tcp_addr))
        .await
        .expect("TCP connect to front router timed out")
        .unwrap();

    let query = build_query(0x4242, "tcp.test.example");
    let mut framed = Vec::with_capacity(query.len() + 2);
    framed.extend_from_slice(&(query.len() as u16).to_be_bytes());
    framed.extend_from_slice(&query);
    client.write_all(&framed).await.unwrap();

    let mut header = [0u8; 2];
    tokio::time::timeout(TIMEOUT, client.read_exact(&mut header))
        .await
        .expect("no TCP answer from front router")
        .unwrap();
    let len = u16::from_be_bytes(header) as usize;
    let mut response = vec![0u8; len];
    client.read_exact(&mut response).await.unwrap();

    assert_eq!(response[0], 0x42);
    assert_eq!(response[1], 0x42);
    assert!(response[2] & 0x80 != 0);
}

#[tokio::test]
async fn test_unmatched_query_gets_refused_when_no_default_backend() {
    let handle = DnsFrontServer::new("127.0.0.1:0".parse().unwrap(), DnsRouter::new())
        .start()
        .await
        .unwrap();

    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    client.connect(handle.udp_addr).await.unwrap();

    let query = build_query(0x0BAD, "unknown.invalid");
    client.send(&query).await.unwrap();

    let mut response = vec![0u8; 512];
    let n = tokio::time::timeout(TIMEOUT, client.recv(&mut response))
        .await
        .expect("unmatched query must still receive REFUSED")
        .unwrap();

    // RCODE = REFUSED (low nibble of byte 3 == 5)
    assert_eq!(response[3] & 0x0F, 0x05);
    assert!(response[2] & 0x80 != 0);
    assert_eq!(&response[6..12], &[0, 0, 0, 0, 0, 0]); // no answers
    assert!(n >= 12);
}

#[tokio::test]
async fn test_default_backend_catches_unmatched_queries() {
    let default_backend = spawn_udp_echo_backend().await;

    let mut router = DnsRouter::new();
    router.set_default_backend(default_backend);

    let handle = DnsFrontServer::new("127.0.0.1:0".parse().unwrap(), router)
        .start()
        .await
        .unwrap();

    let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    client.connect(handle.udp_addr).await.unwrap();

    let query = build_query(0x1111, "anything.org");
    client.send(&query).await.unwrap();

    let mut response = vec![0u8; 512];
    let n = tokio::time::timeout(TIMEOUT, client.recv(&mut response))
        .await
        .expect("default backend was configured, query must be forwarded")
        .unwrap();

    assert_eq!(response[0], 0x11);
    assert_eq!(response[1], 0x11);
    assert!(n >= 12);
}

#[test]
fn test_refused_response_construction() {
    let query = build_query(0xCAFE, "blocked.example.com");
    let refused = build_refused_response(&query);

    assert_eq!(refused[0], 0xCA);
    assert_eq!(refused[1], 0xFE);
    assert_eq!(refused[3] & 0x0F, 0x05);
    assert_eq!(refused[4], 0x00); // QDCOUNT kept (question echoed)
    assert_eq!(refused[5], 0x01);
    assert_eq!(&refused[6..12], &[0, 0, 0, 0, 0, 0]);
    assert_eq!(refused.len(), query.len());

    // Too-short packets produce no response at all
    assert!(build_refused_response(&[0u8; 4]).is_empty());
}
