# Luffy Gateway

<p align="center">
  <strong>Self-Hosted Rust Anti-Censorship Control Panel & DNS Router</strong><br>
  Bilingual Persian (RTL) & English (LTR) · Zero-CDN · Multi-Engine Multiplexer
</p>

---

## Highlights

- **4 Circumvention Engines:** Manages [Soar](https://github.com/getlantern/soar), [MasterDnsVPN](https://github.com/masterking32/MasterDnsVPN), [SlipGate](https://github.com/anonvector/slipgate), and [GooseRelay](https://github.com/Kianmhz/GooseRelayVPN).
- **DNS Front Router (`luffy-router`):** Binds to port 53 (UDP/TCP) and intelligently forwards queries by longest-suffix matching to loopback backends without altering packets or breaking MTU.
- **Unified Subscriptions:** One private link and QR code per user providing dynamic profiles (`?format=luffy`, `slipnet://`, TOML, JSON, or interactive HTML landing page).
- **Honest Enforcement:** Token-bucket bandwidth throttling, stream caps, and automatic cutoff on quota exhaustion or expiration.
- **DNS Doctor:** 7-point automated diagnostic suite detecting misconfigurations, port conflicts, and Cloudflare Orange-Cloud proxy hijacks.
- **100% Rust & Clean-Room:** Built entirely in Rust without Node.js or Python; zero code or assets copied from proprietary panels.

---

## Quickstart

```bash
# One-command installation on Ubuntu/Debian
curl -fsSL https://get.luffy-gateway.org/install.sh | bash

# Run diagnostics
luffy doctor example.com
luffy ports
```

---

## Documentation

- [Verification Matrix & Test Evidence](VERIFICATION.md)
- [Field Testing Guide for Iranian Carriers](FIELD-TEST.md)
- [System Architecture](docs/ARCHITECTURE.md)
- [Upstream Inventory & Hashing](docs/UPSTREAMS.md)
- [Architecture Decision Records (ADRs)](docs/ADR/)
- [User Guide (فارسی)](docs/USER_GUIDE_FA.md)
- [User Guide (English)](docs/USER_GUIDE_EN.md)

---

## Attribution & License

Licensed under [Apache License 2.0](NOTICE).  
Upstream projects:
- **Lantern Soar:** Apache-2.0
- **MasterDnsVPN:** MIT
- **SlipGate / SlipNet:** AGPL-3.0 (standalone CLI execution)
- **GooseRelay / MooseRelay:** MIT
- **Vazirmatn:** SIL OFL 1.1 by Saber Rastikerdar
