# Luffy Gateway User & Operations Guide (English)

**Luffy Gateway** is an all-in-one anti-censorship control panel and router written in Rust. It provisions, monitors, and issues client profiles for four engines: Soar, MasterDnsVPN, SlipGate, and GooseRelay.

---

## 1. Quick Installation

Execute on a clean Ubuntu 22.04/24.04 or Debian 12/13 VPS:
```bash
curl -fsSL https://get.luffy-gateway.org/install.sh | bash
```

Upon completion, the installer outputs:
- **Panel URL:** `http://<SERVER_IP>:8080/<PRIVATE_TOKEN>`
- **Admin Username:** `admin`
- **One-time Password:** Displayed in stdout.

---

## 2. DNS Zone Configuration

Configure the following records in your DNS manager (e.g., Cloudflare in DNS-only / Grey Cloud mode):
1. `A ns.example.com -> <SERVER_IP>`
2. `NS s.example.com  -> ns.example.com` (Soar Zone)
3. `NS m.example.com  -> ns.example.com` (MasterDnsVPN Zone)
4. `NS t.example.com  -> ns.example.com` (SlipGate Zone)

---

## 3. Diagnostics & DNS Doctor

Run the comprehensive 7-point diagnostic tool anytime via CLI:
```bash
luffy doctor example.com
luffy ports
```
This detects port conflicts, Cloudflare proxy interference, and delegation health.

---

## 4. User Provisioning & Subscriptions

1. Navigate to **User Management** in the web panel.
2. Configure quotas, bandwidth limits, stream caps, and select desired engines.
3. Share the private `/sub/<token>` URL or QR code with the client.
4. Clients can import using `?format=luffy`, `?format=slipnet`, `?format=soar`, `?format=masterdns`, or view the interactive HTML landing page.
