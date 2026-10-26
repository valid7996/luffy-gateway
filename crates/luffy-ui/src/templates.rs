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
            nav_html.push_str(&format!(
                r#"<a href="{href}" class="nav-item{active_cls}">{label}</a>"#
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
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 600;
        }}
        .badge-success {{ background: rgba(16, 185, 129, 0.2); color: var(--success); }}
        .badge-warning {{ background: rgba(245, 158, 11, 0.2); color: var(--warning); }}
        .badge-danger {{ background: rgba(239, 68, 68, 0.2); color: var(--danger); }}
        @media (max-width: 768px) {{
            body {{ flex-direction: column; }}
            .sidebar {{ width: 100%; border-inline-end: none; border-bottom: 1px solid var(--border); }}
        }}
    </style>
</head>
<body>
    <aside class="sidebar">
        <div class="brand">{app_name}</div>
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
                <div style="margin-top: 16px; display: flex; gap: 12px; flex-wrap: wrap;">
                    <span class="badge badge-success">Soar (v0.8.2) - Active</span>
                    <span class="badge badge-success">MasterDnsVPN (v1.2.0) - Active</span>
                    <span class="badge badge-success">SlipGate (v1.6.2) - Active</span>
                    <span class="badge badge-success">GooseRelay (v1.1.0) - Active</span>
                </div>
            </div>"#,
            title_users = title_users,
            active_users = active_users,
            title_traffic = title_traffic,
            traffic_mb = traffic_mb,
            title_qps = title_qps,
            router_qps = router_qps,
            engines_title = i18n.translate(lang, "engines")
        );

        Self::render_base_layout(lang, "dashboard", &body, i18n)
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
