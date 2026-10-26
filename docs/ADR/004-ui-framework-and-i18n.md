# ADR 004: UI Framework, Asset Bundling, and Bilingual Localization

## Status
Decided / Accepted

## Context
The panel UI must support:
- Complete bilingual localization: Persian (`fa`, RTL, default) and English (`en`, LTR).
- Light and Dark modes.
- Zero external CDN dependencies (bundled Vazirmatn font with SIL OFL).
- Complete compilation into the single `luffy` binary without requiring Node.js or npm on the target server.

## Decision
1. **Frontend Architecture:**
   - Server-side rendered HTML using Axum + Maud templates paired with reactive vanilla JS components, or Leptos SSR compiled directly with Rust.
   - All CSS, SVGs, and font subsets (`Vazirmatn.woff2`) are embedded directly into the Rust binary at build time using `include_bytes!`/`include_str!`.
2. **Localization Engine:**
   - Project Fluent (`fluent-bundle` in Rust) with zero string literals hardcoded in HTML markup.
   - Complete coverage for validation messages, DNS Doctor diagnostics, QR dialogs, and subscription landing pages.

## Consequences
- Guaranteed zero internet access required during UI rendering.
- Extremely lightweight binary distribution with sub-1MB initial transfer.
