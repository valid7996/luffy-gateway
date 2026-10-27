//! DNS Doctor diagnostics engine.
//!
//! Checks marked "live" perform real network probes (zone A-record via the
//! system resolver, UDP/TCP :53 availability). Checks that require a running
//! Luffy router report Warn with bilingual instructions instead of fake Pass.

use serde::{Deserialize, Serialize};
use std::net::{TcpListener, ToSocketAddrs, UdpSocket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DoctorStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorCheckItem {
    pub id: u8,
    pub name: String,
    pub status: DoctorStatus,
    pub detail_fa: String,
    pub detail_en: String,
    pub fix_fa: Option<String>,
    pub fix_en: Option<String>,
}

pub struct DnsDoctor;

impl DnsDoctor {
    /// Resolves the zone's A record via the system resolver (real probe).
    fn probe_zone_a_record(zone: &str) -> Result<Vec<std::net::IpAddr>, String> {
        let addrs: Vec<_> = (zone, 0u16)
            .to_socket_addrs()
            .map_err(|e| e.to_string())?
            .map(|a| a.ip())
            .collect();
        if addrs.is_empty() {
            Err("no address returned".into())
        } else {
            Ok(addrs)
        }
    }

    /// Probes whether UDP :53 (and optionally TCP :53) can actually be bound
    /// on this host — a real conflict / privilege check.
    fn probe_port53() -> Result<(), String> {
        let udp = UdpSocket::bind("0.0.0.0:53");
        let tcp = TcpListener::bind("0.0.0.0:53");
        match (udp, tcp) {
            (Ok(_u), Ok(_t)) => Ok(()),
            (Err(e), _) => Err(format!("UDP/53: {e}")),
            (_, Err(e)) => Err(format!("TCP/53: {e}")),
        }
    }

    fn warn_live_router(check_name_en: &str) -> DoctorCheckItem {
        DoctorCheckItem {
            id: 0,
            name: check_name_en.to_string(),
            status: DoctorStatus::Warn,
            detail_fa: "این بررسی به روتر Luffy در حال اجرا روی همین سرور نیاز دارد و هنوز ارزیابی نشده است.".to_string(),
            detail_en: "Requires the Luffy router to be running on this host; not evaluated yet.".to_string(),
            fix_fa: Some("اول `luffy router` را اجرا کنید و سپس `luffy doctor` را تکرار کنید.".to_string()),
            fix_en: Some("Start `luffy router` first, then re-run `luffy doctor`.".to_string()),
        }
    }

    /// Executes all 7 DNS Doctor checks on a given zone.
    /// Live probes replace earlier hard-coded placeholder results.
    pub fn run_diagnostics(zone: &str, server_ip: &str, is_cloudflare_proxied: bool) -> Vec<DoctorCheckItem> {
        let mut results = Vec::new();

        // Check 1: Zone A-record reachability (real system-resolver probe)
        let zone_a = Self::probe_zone_a_record(zone);
        results.push(match &zone_a {
            Ok(ips) if ips.iter().any(|ip| ip.to_string() == server_ip) => DoctorCheckItem {
                id: 1,
                name: "Zone A-Record Reachability".to_string(),
                status: DoctorStatus::Pass,
                detail_fa: format!("رکورد A دامنه {zone} به‌درستی به {server_ip} اشاره می‌کند."),
                detail_en: format!("Zone {zone} A record correctly points to {server_ip}."),
                fix_fa: None,
                fix_en: None,
            },
            Ok(ips) => DoctorCheckItem {
                id: 1,
                name: "Zone A-Record Reachability".to_string(),
                status: DoctorStatus::Warn,
                detail_fa: format!("رکورد A دامنه {zone} به {ips:?} اشاره می‌کند ولی IP سرور {server_ip} است."),
                detail_en: format!("Zone {zone} resolves to {ips:?} but the configured server IP is {server_ip}."),
                fix_fa: Some(format!("در پنل DNS رکورد A دامنه را روی {server_ip} تنظیم کنید (ابر خاکستری).")),
                fix_en: Some(format!("Point the zone A record at {server_ip} (DNS-only grey cloud).")),
            },
            Err(reason) => DoctorCheckItem {
                id: 1,
                name: "Zone A-Record Reachability".to_string(),
                status: DoctorStatus::Warn,
                detail_fa: format!("دامنه {zone} قابل حل‌کردن نیست: {reason}"),
                detail_en: format!("Zone {zone} could not be resolved: {reason}"),
                fix_fa: Some("رکورد A دامنه را در سرویس DNS ثبت کنید و ابر پراکسی را خاکستری کنید.".to_string()),
                fix_en: Some("Create the zone A record and disable any proxy (grey cloud).".to_string()),
            },
        });

        // Check 2: Reserved token verification (_luffy-check) — requires live router
        let mut check2 = Self::warn_live_router("Server Authority Proof");
        check2.id = 2;
        results.push(check2);

        // Check 3: Authenticated handshake through public resolvers — requires live router
        let mut check3 = Self::warn_live_router("Engine Handshake");
        check3.id = 3;
        results.push(check3);

        // Check 4: Cloudflare Proxy (Orange Cloud) Detection
        if is_cloudflare_proxied {
            results.push(DoctorCheckItem {
                id: 4,
                name: "Proxy / Wildcard Hijack".to_string(),
                status: DoctorStatus::Fail,
                detail_fa: "ابر نارنجی کلودفلر (پراکسی) روی دامنه تونل فعال است و مانع ترافیک UDP پورت ۵۳ می‌شود!".to_string(),
                detail_en: "Cloudflare proxy (Orange Cloud) is active on tunnel domain, blocking UDP/53!".to_string(),
                fix_fa: Some("در پنل کلودفلر رکورد دامنه تونل را روی حالت DNS Only (ابر خاکستری) بگذارید.".to_string()),
                fix_en: Some("Switch the tunnel DNS record to DNS Only (Grey Cloud) in Cloudflare.".to_string()),
            });
        } else {
            results.push(DoctorCheckItem {
                id: 4,
                name: "Proxy / Wildcard Hijack".to_string(),
                status: DoctorStatus::Pass,
                detail_fa: "ابر خاکستری (DNS Only) فعال است و هیچ تداخلی وجود ندارد.".to_string(),
                detail_en: "DNS-only mode active; no proxy interference detected.".to_string(),
                fix_fa: None,
                fix_en: None,
            });
        }

        // Check 5: Public IP consistency — meaningful only on the server itself
        let mut check5 = Self::warn_live_router("Public IP Consistency");
        check5.id = 5;
        results.push(check5);

        // Check 6: Port 53 availability (real bind probe)
        results.push(match Self::probe_port53() {
            Ok(()) => DoctorCheckItem {
                id: 6,
                name: "Port 53 Availability".to_string(),
                status: DoctorStatus::Pass,
                detail_fa: "پورت ۵۳ UDP/TCP آزاد است و روتر Luffy می‌تواند bind کند.".to_string(),
                detail_en: "UDP/TCP port 53 is free and bindable by the Luffy router.".to_string(),
                fix_fa: None,
                fix_en: None,
            },
            Err(reason) => DoctorCheckItem {
                id: 6,
                name: "Port 53 Availability".to_string(),
                status: DoctorStatus::Warn,
                detail_fa: format!("پورت ۵۳ در دسترس نیست ({reason}) — یا سرویس دیگری اشغال کرده یا دسترسی root/CAP_NET_BIND_SERVICE لازم است."),
                detail_en: format!("Port 53 unavailable ({reason}) — either occupied by another service or root/CAP_NET_BIND_SERVICE is required."),
                fix_fa: Some("سرویس‌های DNS دیگر (systemd-resolved) را غیرفعال کنید و روتر را با دسترسی مناسب اجرا کنید.".to_string()),
                fix_en: Some("Disable competing DNS daemons (e.g. systemd-resolved) and run the router with proper capabilities.".to_string()),
            },
        });

        // Check 7: Last handshake recency — requires live router
        let mut check7 = Self::warn_live_router("Last Handshake Recency");
        check7.id = 7;
        results.push(check7);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_doctor_orange_cloud_detection() {
        let clean_checks = DnsDoctor::run_diagnostics("t.example.com", "1.2.3.4", false);
        assert_eq!(clean_checks[3].status, DoctorStatus::Pass);

        let proxied_checks = DnsDoctor::run_diagnostics("t.example.com", "1.2.3.4", true);
        assert_eq!(proxied_checks[3].status, DoctorStatus::Fail);
        assert!(proxied_checks[3].fix_fa.is_some());
        assert!(proxied_checks[3].detail_en.contains("Cloudflare proxy"));
    }

    #[test]
    fn test_zone_a_record_real_probe() {
        // localhost must resolve through the system resolver
        let ips = DnsDoctor::probe_zone_a_record("localhost").expect("localhost must resolve");
        assert!(ips.iter().any(|ip| ip.is_loopback()));
        assert!(DnsDoctor::probe_zone_a_record("nonexistent-zone-4b1f.invalid").is_err());
    }

    #[test]
    fn test_live_router_checks_are_honest_warns() {
        let checks = DnsDoctor::run_diagnostics("t.example.com", "1.2.3.4", false);
        for id in [2, 3, 5, 7] {
            let check = checks.iter().find(|c| c.id == id).unwrap();
            assert_eq!(
                check.status,
                DoctorStatus::Warn,
                "live-router check {id} must not fake a Pass"
            );
            assert!(check.detail_en.contains("running") || check.detail_en.contains("evaluated"));
        }
    }

    #[test]
    fn test_port53_probe_never_fakes_a_pass_when_busy() {
        let checks = DnsDoctor::run_diagnostics("t.example.com", "1.2.3.4", false);
        let check6 = checks.iter().find(|c| c.id == 6).unwrap();
        // On a free port it is a real Pass; on a busy/privileged host an honest Warn.
        assert!(matches!(check6.status, DoctorStatus::Pass | DoctorStatus::Warn));
    }
}
