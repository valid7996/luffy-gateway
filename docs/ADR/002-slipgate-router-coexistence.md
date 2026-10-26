# ADR 002: SlipGate and DNS Router Coexistence

## Status
Decided / Accepted

## Context
SlipGate features its own internal router (`slipgate-dnsrouter`). However, on a Luffy-managed server, port 53 (UDP/TCP) must be shared across all 4 censorship circumvention engines (Soar, MasterDnsVPN, SlipGate, and GooseRelay DNS transports).

## Decision
1. **Luffy Router owns port 53:** `luffy-router` will exclusively bind to public `0.0.0.0:53` (UDP/TCP).
2. **SlipGate Multi-Mode:** SlipGate will run in multi-instance mode where its individual tunnel transports (dnstt, vaydns, slipstream) listen on discrete loopback ports (`127.0.0.1:531x`, `127.0.0.1:532x`).
3. **Dispatch:** `luffy-router` inspects the first DNS question, extracts the QNAME, performs longest-suffix matching, and forwards packets intact to the assigned engine loopback port without altering query payloads or breaking tunnel MTU.

## Consequences
- Single entrypoint for all DNS tunnels.
- Zero port conflicts between engines.
