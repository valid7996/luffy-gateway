# Enforcement Matrix (Phase 0 Baseline)

This matrix defines the enforcement capabilities for each circumvention engine as planned in Milestone 0.

| Engine | Quota Enforcement | Expiry Enforcement | Speed Limit (Token Bucket) | Stream Limit | Session Revocation | UI Badge Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Soar** | Router byte-counter drop on zone | Fake/real clock route removal | Best-effort (router rate limit) | 128 concurrent streams | Zone revocation in router (<= 5s) | **Best-effort** |
| **MasterDnsVPN** | Egress SOCKS metering | Periodic health & credential check | Token bucket on external SOCKS5 | Per-server client limits | Key rotation & process restart | **Best-effort** / **Enforced** (via isolated instance) |
| **SlipGate** | User accounting via CLI/proxy | Dynamic credential revocation | Token bucket on SOCKS/SSH backend | Controlled via backend worker caps | CLI `users remove` / restart | **Enforced** |
| **GooseRelay** | SOCKS5 metering or metrics count | Expiration timer revokes script keys | Token bucket on local SOCKS5 proxy | Worker cap in client config | Script key rotation | **Best-effort** (or **Not supported** on shared setup) |
