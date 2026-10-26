# Luffy Gateway Architecture

## Overview
Luffy Gateway is a high-performance, single-binary censorship-circumvention panel and router implemented entirely in Rust. It multiplexes four specialized anti-censorship engines through a unified port 53 front-router and centralized traffic metering.

## Workspace Layout
```
crates/
├── luffy-core/        # Domain types, settings registry (typed, bilingual), validation, quota math
├── luffy-router/      # High-throughput DNS Front Router owning UDP/TCP :53
├── luffy-engines/     # Adapters for Soar, MasterDnsVPN, SlipGate, GooseRelay, slipnet:// codec
├── luffy-meter/       # Token-bucket speed limiters, atomic byte counters, stream caps
├── luffy-panel/       # Web server, authentication, REST API, subscription multi-format exporter
├── luffy-ui/          # Zero-CDN templates, dark-first design system, Persian (RTL) & English (LTR)
├── luffy-cli/         # luffy executable, installer, DNS Doctor, ports inspector, backup
└── luffy-testkit/     # Iran-like lab simulation stubs (S0 to S7 impairment scenarios)
```

## Data Flow
```
Incoming Client DNS Query (:53 UDP/TCP)
                   │
                   ▼
     ┌───────────────────────────┐
     │   luffy-router (:53)      │  <── Parses QNAME, matches longest suffix
     └─────────────┬─────────────┘
                   │
       ┌───────────┼───────────┬───────────┐
       ▼           ▼           ▼           ▼
  127.0.0.1:5310 127.0.0.1:5320 127.0.0.1:5330 127.0.0.1:8443
    [Soar]      [MasterDNS]   [SlipGate]   [GooseRelay]
```

## Security Model
- **Process Isolation:** Engines execute under isolated unprivileged systemd services.
- **Port 53 Exclusivity:** Handled by `luffy-router` with `CAP_NET_BIND_SERVICE`.
- **Clean-Room Assurance:** Zero proprietary code or asset reuse from WhiteDNS.
- **AGPL-3.0 Compliance:** SlipGate executed strictly as a standalone CLI process.
