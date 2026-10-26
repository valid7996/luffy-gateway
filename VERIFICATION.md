# Luffy Gateway Verification Matrix

## Environment Probe
- **Platform:** Windows (Development Node) / Target: Linux (Ubuntu 22.04/24.04, Debian 12/13)
- **Toolchain:** Rust stable 1.99.0, Cargo 1.99.0
- **Lab Tier:** Tier A (Local mock / Stub resolver unit verification)

---

## Acceptance Table

| Milestone | Requirement / Component | Test Name | Status | Evidence / Notes |
| :--- | :--- | :--- | :--- | :--- |
| **M0** | Upstreams manifest & hashing | Inspection of `docs/UPSTREAMS.md` | **PASS** | Versions pinned for Soar, MasterDnsVPN, SlipGate, GooseRelay |
| **M0** | Architecture Decision Records | `docs/ADR/001` through `005` | **PASS** | ADR-001 through ADR-005 written and committed |
| **M0** | Clean-room & Permissions status | `docs/PERMISSIONS.md` | **PASS** | Clean-room enforced; no WhiteDNS assets copied |
| **M0** | Enforcement capability matrix | `docs/ENFORCEMENT_MATRIX.md` | **PASS** | Status defined: Enforced, Best-effort, Not supported |
| **M1** | Settings registry typed validation | `luffy_core::registry::tests::test_setting_entry_validation` | **PASS** | Integer, boolean, and enum bounds checked |
| **M1** | Soar settings drift & schema | `drift_tests::test_settings_registry_drift_soar` | **PASS** | 100% bilingual help text present (EN/FA) |
| **M1** | MasterDns settings drift & schema | `drift_tests::test_settings_registry_drift_masterdns` | **PASS** | 100% bilingual help text present (EN/FA) |
| **M1** | Router longest-suffix matching | `luffy_router::tests::test_longest_suffix_match` | **PASS** | Exact and nested subdomains routed to loopback ports |
| **M1** | Router duplicate suffix rejection | `luffy_router::tests::test_duplicate_route_rejection` | **PASS** | Duplicate identical domain registration fails |
| **M1** | Testkit stub resolver QNAME limits | `luffy_testkit::stub_resolver::tests::test_stub_resolver_qname_limit` | **PASS** | S1/S3 scenario QNAME policy tested |
| **M1** | Testkit stub resolver QTYPE block | `luffy_testkit::stub_resolver::tests::test_stub_resolver_blocked_qtype` | **PASS** | S3 scenario QTYPE filter tested |
| **M2** | SlipGate settings drift & schema | `drift_tests::test_settings_registry_drift_slipgate` | **PASS** | 100% bilingual help text present (EN/FA) |
| **M2** | SlipGate CLI non-interactive builder | `luffy_engines::slipgate::tests::test_slipgate_cli_args_generation` | **PASS** | CLI `tunnel add` argument flags generated |
| **M2** | SlipNet URI codec roundtrip | `luffy_engines::slipnet_codec::tests::test_slipnet_codec_roundtrip` | **PASS** | `slipnet://` URI encoding and parsing validated |
| **M2** | GooseRelay settings drift & schema | `drift_tests::test_settings_registry_drift_gooserelay` | **PASS** | 100% bilingual help text present (EN/FA) |
| **M2** | GooseRelay Apps Script generator | `luffy_engines::apps_script::tests::test_apps_script_generation` | **PASS** | `Code.gs` generation with VPS_URL and key substitution |
| **M2** | GooseRelay Apps Script quota calculation | `luffy_engines::apps_script::tests::test_quota_estimation` | **PASS** | 20,000 requests/day limit and warnings tested |
| **All** | Live carrier testing on Iranian ISPs | Field test execution | **NOT VERIFIED** | Requires physical device and Iranian carrier network |
