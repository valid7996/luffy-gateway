# ADR 003: Per-User Isolation and Metering Architecture

## Status
Decided / Accepted

## Context
Upstream engines differ significantly in their native multi-user and traffic-accounting capabilities:
- **SlipGate:** Native SSH and SOCKS5 user credentials.
- **MasterDnsVPN:** Single server key in upstream; per-server client limits exist, but fine-grained per-user accounting is not natively implemented.
- **Soar:** Single key per zone.
- **GooseRelay:** Shared secret tunnel key.

## Decision
1. **Multi-layer Metering:**
   - **Router-level:** `luffy-router` counts incoming/outgoing bytes per sub-domain zone allocation for DNS tunnels.
   - **Egress-level:** When external SOCKS5 chaining is active, `luffy-meter` measures TCP payload bytes directly.
2. **Honest Status Display:**
   - Where exact quota and speed enforcement is verified: label as **Enforced**.
   - Where rate-limiting is approximate or egress-only: label as **Best-effort**.
   - Where shared-key engines do not provide isolation without spawning multiple instances: label as **Not supported** unless an isolated instance is configured.

## Consequences
- Operator is never misled about security or quota enforcement boundaries.
