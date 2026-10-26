//! Clean-room responsive HTML/CSS templates for the Luffy Gateway panel.

use crate::i18n::I18nManager;

pub struct PanelTemplates;

impl PanelTemplates {
    pub fn render_base_layout(
        lang: &str,
        active_nav: &str,
        content_body: &str,
        i18n: &I18nManager,
    ) -> String {
        let is_fa = lang.starts_with("fa");
        let dir = if is_fa { "rtl" } else { "ltr" };
        let app_name = i18n.translate(lang, "app_name");

        let nav_items = [
            ("dashboard", i18n.translate(lang, "dashboard"), "/dashboard"),
            ("engines", i18n.translate(lang, "engines"), "/engines"),
            ("domains", i18n.translate(lang, "domains"), "/domains"),
            ("ports", i18n.translate(lang, "ports"), "/ports"),
            ("users", i18n.translate(lang, "users"), "/users"),
            ("about", i18n.translate(lang, "about"), "/about"),
        ];

        let mut nav_html = String::new();
        for (id, label, href) in nav_items {
            let active_cls = if id == active_nav { " active" } else { "" };
            let lang_query = if is_fa { "?lang=fa" } else { "?lang=en" };
            nav_html.push_str(&format!(
                r#"<a href="{href}{lang_query}" class="nav-item{active_cls}">{label}</a>"#
            ));
        }

        let lang_toggle = if is_fa {
            r#"<a href="?lang=en" class="btn-lang">English (LTR)</a>"#
        } else {
            r#"<a href="?lang=fa" class="btn-lang">فارسی (RTL)</a>"#
        };

        format!(
            r#"<!DOCTYPE html>
<html lang="{lang}" dir="{dir}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{app_name}</title>
    <style>
        :root {{
            --bg-base: #0b0f19;
            --bg-surface: #151d2f;
            --bg-card: #1c263c;
            --text-primary: #f8fafc;
            --text-secondary: #94a3b8;
            --accent: #38bdf8;
            --accent-hover: #0284c7;
            --border: #2e3d5c;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: system-ui, -apple-system, 'Vazirmatn', sans-serif;
            background: var(--bg-base);
            color: var(--text-primary);
            line-height: 1.5;
            min-height: 100vh;
            display: flex;
        }}
        .sidebar {{
            width: 250px;
            background: var(--bg-surface);
            border-inline-end: 1px solid var(--border);
            padding: 24px 16px;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }}
        .brand {{
            font-size: 1.25rem;
            font-weight: 700;
            color: var(--accent);
            margin-bottom: 24px;
            padding-inline: 8px;
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .nav-item {{
            display: block;
            padding: 10px 14px;
            border-radius: 6px;
            color: var(--text-secondary);
            text-decoration: none;
            transition: all 0.2s;
        }}
        .nav-item:hover {{
            background: var(--bg-card);
            color: var(--text-primary);
        }}
        .nav-item.active {{
            background: var(--accent);
            color: #ffffff;
            font-weight: 600;
        }}
        .main-content {{
            flex: 1;
            display: flex;
            flex-direction: column;
            overflow-y: auto;
        }}
        .header {{
            height: 64px;
            background: var(--bg-surface);
            border-bottom: 1px solid var(--border);
            padding: 0 24px;
            display: flex;
            align-items: center;
            justify-content: flex-end;
            gap: 16px;
        }}
        .btn-lang {{
            background: var(--bg-card);
            color: var(--accent);
            padding: 6px 12px;
            border-radius: 4px;
            text-decoration: none;
            font-size: 0.875rem;
            border: 1px solid var(--border);
            font-weight: 500;
        }}
        .content-body {{
            padding: 24px;
            max-width: 1200px;
            width: 100%;
        }}
        .grid-cards {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }}
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
        }}
        .card-title {{
            font-size: 0.875rem;
            color: var(--text-secondary);
            margin-bottom: 8px;
        }}
        .card-value {{
            font-size: 1.75rem;
            font-weight: 700;
            color: var(--text-primary);
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 8px;
            overflow: hidden;
            margin-top: 12px;
        }}
        th, td {{
            padding: 12px 16px;
            text-align: start;
            border-bottom: 1px solid var(--border);
        }}
        th {{
            background: var(--bg-card);
            color: var(--text-secondary);
            font-weight: 600;
        }}
        .badge {{
            display: inline-block;
            padding: 4px 10px;
            border-radius: 4px;
            font-size: 0.8rem;
            font-weight: 600;
        }}
        .badge-success {{ background: rgba(16, 185, 129, 0.2); color: var(--success); }}
        .badge-warning {{ background: rgba(245, 158, 11, 0.2); color: var(--warning); }}
        .badge-danger {{ background: rgba(239, 68, 68, 0.2); color: var(--danger); }}
        .btn {{
            background: var(--accent);
            color: #0b0f19;
            padding: 8px 16px;
            border-radius: 6px;
            text-decoration: none;
            font-weight: 600;
            border: none;
            cursor: pointer;
            display: inline-block;
        }}
        .btn-outline {{
            background: transparent;
            color: var(--accent);
            border: 1px solid var(--accent);
        }}
        @media (max-width: 768px) {{
            body {{ flex-direction: column; }}
            .sidebar {{ width: 100%; border-inline-end: none; border-bottom: 1px solid var(--border); }}
        }}
    </style>
</head>
<body>
    <aside class="sidebar">
        <div class="brand">🏴‍☠️ {app_name}</div>
        {nav_html}
    </aside>
    <div class="main-content">
        <header class="header">
            {lang_toggle}
        </header>
        <main class="content-body">
            {content_body}
        </main>
    </div>
</body>
</html>"#
        )
    }

    pub fn render_dashboard(
        lang: &str,
        active_users: u32,
        traffic_mb: u64,
        router_qps: u32,
        i18n: &I18nManager,
    ) -> String {
        let title_users = i18n.translate(lang, "active_users");
        let title_traffic = i18n.translate(lang, "traffic_today");
        let title_qps = i18n.translate(lang, "router_qps");

        let is_fa = lang.starts_with("fa");
        let engines_desc = if is_fa {
            "وضعیت همزمان ۴ موتور فیلترشکن متصل به روتر پورت ۵۳"
        } else {
            "Simultaneous status of 4 anti-censorship engines bound to port 53 front router"
        };

        let body = format!(
            r#"<div class="grid-cards">
                <div class="card">
                    <div class="card-title">{title_users}</div>
                    <div class="card-value">{active_users}</div>
                </div>
                <div class="card">
                    <div class="card-title">{title_traffic}</div>
                    <div class="card-value">{traffic_mb} MB</div>
                </div>
                <div class="card">
                    <div class="card-title">{title_qps}</div>
                    <div class="card-value">{router_qps} /s</div>
                </div>
            </div>
            <div class="card">
                <h2>{engines_title}</h2>
                <p style="color: var(--text-secondary); margin-top: 4px;">{engines_desc}</p>
                <div style="margin-top: 20px; display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 16px;">
                    <div style="background: var(--bg-card); padding: 16px; border-radius: 8px; border: 1px solid var(--border);">
                        <h4>Soar Tunnel</h4>
                        <p style="color: var(--text-secondary); font-size: 0.85rem; margin: 4px 0 12px;">Port 5310 · TCP-only DNS</p>
                        <span class="badge badge-success">Active (v0.8.2)</span>
                    </div>
                    <div style="background: var(--bg-card); padding: 16px; border-radius: 8px; border: 1px solid var(--border);">
                        <h4>MasterDnsVPN</h4>
                        <p style="color: var(--text-secondary); font-size: 0.85rem; margin: 4px 0 12px;">Port 5320 · ChaCha20 / ARQ</p>
                        <span class="badge badge-success">Active (v1.2.0)</span>
                    </div>
                    <div style="background: var(--bg-card); padding: 16px; border-radius: 8px; border: 1px solid var(--border);">
                        <h4>SlipGate / SlipNet</h4>
                        <p style="color: var(--text-secondary); font-size: 0.85rem; margin: 4px 0 12px;">Port 5330 · DNSTT / NoizDNS</p>
                        <span class="badge badge-success">Active (v1.6.2)</span>
                    </div>
                    <div style="background: var(--bg-card); padding: 16px; border-radius: 8px; border: 1px solid var(--border);">
                        <h4>GooseRelay</h4>
                        <p style="color: var(--text-secondary); font-size: 0.85rem; margin: 4px 0 12px;">Port 8443 · Google Script</p>
                        <span class="badge badge-success">Active (v1.1.0)</span>
                    </div>
                </div>
            </div>"#,
            title_users = title_users,
            active_users = active_users,
            title_traffic = title_traffic,
            traffic_mb = traffic_mb,
            title_qps = title_qps,
            router_qps = router_qps,
            engines_title = i18n.translate(lang, "engines"),
            engines_desc = engines_desc
        );

        Self::render_base_layout(lang, "dashboard", &body, i18n)
    }

    pub fn render_engines(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");
        let _title = if is_fa { "مدیریت و پیکربندی موتورها" } else { "Engines Management & Tuning" };

        let content = if is_fa {
            r#"<div class="card">
                <h2>مدیریت موتورهای فیلترشکن</h2>
                <table>
                    <thead>
                        <tr><th>موتور</th><th>پورت لوپ‌بک</th><th>لایسنس</th><th>روش انکپسولیشن</th><th>وضعیت</th></tr>
                    </thead>
                    <tbody>
                        <tr><td><strong>Soar</strong></td><td>127.0.0.1:5310</td><td>Apache-2.0</td><td>TCP-over-DNS (QNAME 101-253)</td><td><span class="badge badge-success">روشن</span></td></tr>
                        <tr><td><strong>MasterDnsVPN</strong></td><td>127.0.0.1:5320</td><td>MIT</td><td>ChaCha20 / 8-Resolver Balance</td><td><span class="badge badge-success">روشن</span></td></tr>
                        <tr><td><strong>SlipGate</strong></td><td>127.0.0.1:5330</td><td>AGPL-3.0</td><td>DNSTT / NoizDNS / VayDNS</td><td><span class="badge badge-success">روشن</span></td></tr>
                        <tr><td><strong>GooseRelay</strong></td><td>0.0.0.0:8443</td><td>MIT</td><td>Google Apps Script Proxy</td><td><span class="badge badge-success">روشن</span></td></tr>
                    </tbody>
                </table>
            </div>"#
        } else {
            r#"<div class="card">
                <h2>Engines Management</h2>
                <table>
                    <thead>
                        <tr><th>Engine</th><th>Loopback Port</th><th>License</th><th>Encapsulation</th><th>Status</th></tr>
                    </thead>
                    <tbody>
                        <tr><td><strong>Soar</strong></td><td>127.0.0.1:5310</td><td>Apache-2.0</td><td>TCP-over-DNS (QNAME 101-253)</td><td><span class="badge badge-success">RUNNING</span></td></tr>
                        <tr><td><strong>MasterDnsVPN</strong></td><td>127.0.0.1:5320</td><td>MIT</td><td>ChaCha20 / 8-Resolver Balance</td><td><span class="badge badge-success">RUNNING</span></td></tr>
                        <tr><td><strong>SlipGate</strong></td><td>127.0.0.1:5330</td><td>AGPL-3.0</td><td>DNSTT / NoizDNS / VayDNS</td><td><span class="badge badge-success">RUNNING</span></td></tr>
                        <tr><td><strong>GooseRelay</strong></td><td>0.0.0.0:8443</td><td>MIT</td><td>Google Apps Script Proxy</td><td><span class="badge badge-success">RUNNING</span></td></tr>
                    </tbody>
                </table>
            </div>"#
        };

        Self::render_base_layout(lang, "engines", content, i18n)
    }

    pub fn render_domains(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");
        let content = if is_fa {
            r#"<div class="card">
                <h2>دامنه‌ها و پزشک دی‌ان‌اس (DNS Doctor)</h2>
                <p style="color: var(--text-secondary); margin: 8px 0 16px;">
                    روتر فرانت Luffy بسته‌ها را بر اساس طولانی‌ترین پسوند به موتور مربوطه هدایت می‌کند.
                </p>
                <table>
                    <thead>
                        <tr><th>نوع سرویس</th><th>ساب‌دامین نمونه</th><th>رکورد والد</th><th>وضعیت پروکسی کلودفلر</th><th>پزشک دی‌ان‌اس</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>Soar Zone</td><td>s.example.com</td><td>NS -> ns.example.com</td><td>خاکستری (DNS Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>MasterDNS Zone</td><td>m.example.com</td><td>NS -> ns.example.com</td><td>خاکستری (DNS Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>SlipGate Zone</td><td>t.example.com</td><td>NS -> ns.example.com</td><td>خاکستری (DNS Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>Panel Host</td><td>panel.example.com</td><td>A -> سرور</td><td>اختیاری</td><td><span class="badge badge-success">PASS</span></td></tr>
                    </tbody>
                </table>
            </div>"#
        } else {
            r#"<div class="card">
                <h2>Domains & DNS Doctor</h2>
                <p style="color: var(--text-secondary); margin: 8px 0 16px;">
                    Luffy front router routes packets using longest-suffix matching to assigned backends.
                </p>
                <table>
                    <thead>
                        <tr><th>Engine Role</th><th>Zone Subdomain</th><th>Parent Record</th><th>Cloudflare Proxy</th><th>DNS Doctor</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>Soar Zone</td><td>s.example.com</td><td>NS -> ns.example.com</td><td>Grey Cloud (DNS-Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>MasterDNS Zone</td><td>m.example.com</td><td>NS -> ns.example.com</td><td>Grey Cloud (DNS-Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>SlipGate Zone</td><td>t.example.com</td><td>NS -> ns.example.com</td><td>Grey Cloud (DNS-Only)</td><td><span class="badge badge-success">PASS</span></td></tr>
                        <tr><td>Panel Host</td><td>panel.example.com</td><td>A -> Server IP</td><td>Optional</td><td><span class="badge badge-success">PASS</span></td></tr>
                    </tbody>
                </table>
            </div>"#
        };

        Self::render_base_layout(lang, "domains", content, i18n)
    }

    pub fn render_ports(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");
        let content = if is_fa {
            r#"<div class="card">
                <h2>پورت‌ها و فایروال سرور</h2>
                <table>
                    <thead>
                        <tr><th>پورت</th><th>پروتکل</th><th>سرویس</th><th>وضعیت شنود</th><th>قانون فایروال</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>53</td><td>UDP / TCP</td><td>روتر فرانت Luffy (Soar/Master/Slip)</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>80</td><td>TCP</td><td>چالش ACME Let's Encrypt</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>443</td><td>TCP</td><td>پنل HTTPS / NaiveProxy</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>8443</td><td>TCP</td><td>سرور خروجی GooseRelay</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>22</td><td>TCP</td><td>اتصال امن SSH</td><td><span class="badge badge-success">LISTEN</span></td><td>محافظت‌شده</td></tr>
                    </tbody>
                </table>
            </div>"#
        } else {
            r#"<div class="card">
                <h2>Managed Ports & Firewall Status</h2>
                <table>
                    <thead>
                        <tr><th>Port</th><th>Protocol</th><th>Service</th><th>Listening</th><th>Firewall Ownership</th></tr>
                    </thead>
                    <tbody>
                        <tr><td>53</td><td>UDP / TCP</td><td>Luffy DNS Router (Soar/Master/Slip)</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>80</td><td>TCP</td><td>ACME HTTP Challenge</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>443</td><td>TCP</td><td>Panel HTTPS / NaiveProxy</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>8443</td><td>TCP</td><td>GooseRelay Exit Server</td><td><span class="badge badge-success">LISTEN</span></td><td>luffy-managed</td></tr>
                        <tr><td>22</td><td>TCP</td><td>SSH Access</td><td><span class="badge badge-success">LISTEN</span></td><td>keep-open</td></tr>
                    </tbody>
                </table>
            </div>"#
        };

        Self::render_base_layout(lang, "ports", content, i18n)
    }

    pub fn render_users(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");
        let content = if is_fa {
            r#"<div class="card">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                    <h2>حساب‌های کاربری و اشتراک‌ها</h2>
                    <button class="btn" onclick="alert('قابلیت افزودن کاربر فعال است')">+ افزودن کاربر جدید</button>
                </div>
                <table>
                    <thead>
                        <tr><th>کاربر</th><th>سرویس‌های مجاز</th><th>سهمیه حجم</th><th>محدودیت سرعت</th><th>استریم همزمان</th><th>لینک اشتراک</th></tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><strong>vip_user1</strong></td>
                            <td><span class="badge badge-success">Soar</span> <span class="badge badge-success">SlipGate</span></td>
                            <td>20 GB</td>
                            <td>4000 kbps</td>
                            <td>3 استریم</td>
                            <td><a href="/sub/demo_token?format=html" target="_blank" class="btn btn-outline" style="padding: 4px 8px; font-size: 0.8rem;">مشاهده صفحه ساب</a></td>
                        </tr>
                        <tr>
                            <td><strong>office_team</strong></td>
                            <td><span class="badge badge-success">همه موتورها</span></td>
                            <td>100 GB</td>
                            <td>نامحدود</td>
                            <td>10 استریم</td>
                            <td><a href="/sub/demo_token?format=html" target="_blank" class="btn btn-outline" style="padding: 4px 8px; font-size: 0.8rem;">مشاهده صفحه ساب</a></td>
                        </tr>
                    </tbody>
                </table>
            </div>"#
        } else {
            r#"<div class="card">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                    <h2>Users & Subscriptions</h2>
                    <button class="btn" onclick="alert('Create user dialog')">+ Add New User</button>
                </div>
                <table>
                    <thead>
                        <tr><th>User</th><th>Entitlements</th><th>Quota</th><th>Speed Limit</th><th>Max Streams</th><th>Subscription</th></tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><strong>vip_user1</strong></td>
                            <td><span class="badge badge-success">Soar</span> <span class="badge badge-success">SlipGate</span></td>
                            <td>20 GB</td>
                            <td>4000 kbps</td>
                            <td>3 streams</td>
                            <td><a href="/sub/demo_token?format=html" target="_blank" class="btn btn-outline" style="padding: 4px 8px; font-size: 0.8rem;">Open Sub Link</a></td>
                        </tr>
                        <tr>
                            <td><strong>office_team</strong></td>
                            <td><span class="badge badge-success">All Engines</span></td>
                            <td>100 GB</td>
                            <td>Unlimited</td>
                            <td>10 streams</td>
                            <td><a href="/sub/demo_token?format=html" target="_blank" class="btn btn-outline" style="padding: 4px 8px; font-size: 0.8rem;">Open Sub Link</a></td>
                        </tr>
                    </tbody>
                </table>
            </div>"#
        };

        Self::render_base_layout(lang, "users", content, i18n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering_rtl_ltr() {
        let i18n = I18nManager::new();

        let html_fa = PanelTemplates::render_dashboard("fa", 15, 1024, 45, &i18n);
        assert!(html_fa.contains("dir=\"rtl\""));
        assert!(html_fa.contains("کاربران فعال"));

        let html_en = PanelTemplates::render_dashboard("en", 15, 1024, 45, &i18n);
        assert!(html_en.contains("dir=\"ltr\""));
        assert!(html_en.contains("Active Users"));
    }

    #[test]
    fn test_size_budget_under_1mb() {
        let i18n = I18nManager::new();
        let html = PanelTemplates::render_dashboard("fa", 100, 5000, 200, &i18n);
        assert!(html.len() < 100_000, "Dashboard size must be under 100KB uncompressed");
    }
}
