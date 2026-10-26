# ADR 001: Soar Server Implementation Choice

## Status
Decided / Accepted

## Context
Soar tunnel upstream provides both a Go implementation (`getlantern/soar`) and a wire-compatible Rust implementation (`getlantern/soar-rs`). Additionally, open-source references exist such as `mahdigholamipak/soar-tunnel`.
Luffy Gateway needs to manage Soar endpoints and provide reliable client profile generation.

## Decision
1. **Primary Runtime:** Luffy will manage the pinned official server binary (`soar-server`) driven via isolated systemd services and loopback ports.
2. **Library Investigation:** For embedded testing and client handshake verification within the testkit, wire-compatible protocol primitives from `soar-rs` will be referenced clean-room.
3. **Multi-account isolation:** Because upstream Soar server natively operates with a single pre-shared key per instance, multi-user isolation is achieved by assigning dedicated subdomains to each account and routing incoming DNS tunnel queries to isolated loopback instances via `luffy-router`.

## Consequences
- Requires Luffy router to cleanly dispatch subdomains to separate loopback ports.
- Guarantees 100% interoperability with official Soar clients.
