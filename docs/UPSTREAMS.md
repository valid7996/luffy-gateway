# Upstream Engines Inventory and Pinning Manifest

| Engine | Upstream Repository | Version / Tag | Licence | Server Binaries Managed | Client Import Scheme | Per-User Accounting Strategy |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Soar** | `getlantern/soar`<br>`getlantern/soar-rs` | v0.8.x (Pinned) | Apache-2.0 | `soar-server` (or `soar-rs`) | Open Soar profile / `soar://` URI & `client_config.toml` | Per-user subdomain delegation via front router or SOCKS metering chain |
| **MasterDnsVPN** | `masterking32/MasterDnsVPN` | Latest stable | MIT | `MasterDnsVPN` server binary | `client_config.toml` + QR code | Isolated loopback instances or metered SOCKS5 upstream |
| **SlipGate** | `anonvector/slipgate` | v1.6.2 (Pinned) | AGPL-3.0 | `slipgate`, `dnstt-server`, `vaydns-server`, `slipstream-server`, `caddy-naive` | `slipnet://` URI + QR code | Separate tunnel credentials / SOCKS / SSH accounts |
| **GooseRelay** | `Kianmhz/GooseRelayVPN`<br>`easyfast2008/MooseRelayVPN` | Pinned commit | MIT | GooseRelay exit server (`:8443`) | `client_config.json` + `Code.gs` | Isolated user instances / ports or Best-effort / Not supported label |

---

## Hash Verification (SHA-256)
All upstream release artifacts must match explicit SHA-256 digests in production manifests before installation.
