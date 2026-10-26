# ADR 005: Licensing Strategy, Third-Party Attribution, and Clean-Room Isolation

## Status
Decided / Accepted

## Context
Multiple upstream projects with varying licenses are utilized:
- **WhiteDNS:** Source-Available Proprietary (explicit prohibition against forks, clones, or derivative competing products without permission).
- **SlipGate:** AGPL-3.0 (strong copyleft requiring complete source release if modified or linked directly).
- **Soar, MasterDnsVPN, GooseRelay:** Apache-2.0, MIT.

## Decision
1. **Clean-Room on WhiteDNS:** Under no circumstances will WhiteDNS source code, assets, UI layouts, or proprietary subscription schemes be used or copied. All behavior is re-implemented clean-room based on RFCs and public protocols.
2. **Process-Level Isolation for SlipGate:** SlipGate is executed strictly as a standalone upstream binary via CLI invocations (`std::process::Command::arg`). No SlipGate source code is linked or integrated into the Luffy Rust codebase. Full license notices and source code offers are distributed alongside releases.
3. **Attribution:** The panel UI "About" section and `THIRD_PARTY_LICENSES` file prominently attribute all upstream maintainers.

## Consequences
- 100% legal compliance and licensing integrity.
- Zero risk of involuntary AGPL contagion across the core Luffy Gateway repository.
