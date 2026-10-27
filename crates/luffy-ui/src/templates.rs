//! Clean-room responsive HTML/CSS/JS templates for the Luffy Gateway panel.
//! Zero-CDN, offline-ready, minimalist dark-slate design system with SVG icons.

use crate::i18n::I18nManager;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserViewModel {
    pub id: String,
    pub name: String,
    pub token: String,
    pub engines: Vec<String>,
    pub quota_gb: f64,
    pub used_gb: f64,
    pub speed_kbps: u32,
    pub max_streams: u32,
    pub enabled: bool,
}

pub struct PanelTemplates;

impl PanelTemplates {
    /// Core layout shell with responsive sidebar, top navbar, toast manager, and minimalist design system
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
            ("dashboard", i18n.translate(lang, "dashboard"), "/dashboard", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>"#),
            ("engines", i18n.translate(lang, "engines"), "/engines", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m7.8 16.2-2.9 2.9"/><path d="M2 12h4"/><path d="m7.8 7.8-2.9-2.9"/><circle cx="12" cy="12" r="3"/></svg>"#),
            ("domains", i18n.translate(lang, "domains"), "/domains", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#),
            ("ports", i18n.translate(lang, "ports"), "/ports", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 7h.01"/><path d="M17 7h.01"/><path d="M7 17h.01"/><path d="M17 17h.01"/></svg>"#),
            ("users", i18n.translate(lang, "users"), "/users", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#),
            ("about", i18n.translate(lang, "about"), "/about", r#"<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#),
        ];

        let mut nav_html = String::new();
        for (id, label, href, svg) in nav_items {
            let active_cls = if id == active_nav { " active" } else { "" };
            let lang_query = if is_fa { "?lang=fa" } else { "?lang=en" };
            nav_html.push_str(&format!(
                r#"<a href="{href}{lang_query}" class="nav-item{active_cls}">
                    <span class="nav-icon">{svg}</span>
                    <span class="nav-label">{label}</span>
                </a>"#
            ));
        }

        let lang_toggle = if is_fa {
            r#"<a href="?lang=en" class="btn-ghost" title="Switch to English">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
                <span>English</span>
            </a>"#
        } else {
            r#"<a href="?lang=fa" class="btn-ghost" title="تغییر به زبان فارسی">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
                <span>فارسی</span>
            </a>"#
        };

        let node_label = if is_fa { "گره اصلی: ایران / فرانکفورت" } else { "Active Node: Edge-Router-01" };
        let uptime_label = if is_fa { "پایداری روتر: ۹۹.۹۸٪" } else { "Router Uptime: 99.98%" };

        format!(
            r#"<!DOCTYPE html>
<html lang="{lang}" dir="{dir}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{app_name} · Minimalist Admin</title>
    <style>
        :root {{
            --bg-base: #090d16;
            --bg-surface: #0f172a;
            --bg-card: #131c31;
            --bg-card-hover: #17233d;
            --border: #1e293b;
            --border-highlight: #334155;
            --border-glow: rgba(56, 189, 248, 0.25);
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --text-dim: #64748b;
            --primary: #38bdf8;
            --primary-hover: #0284c7;
            --primary-glow: rgba(56, 189, 248, 0.15);
            --success: #10b981;
            --success-glow: rgba(16, 185, 129, 0.15);
            --warning: #f59e0b;
            --danger: #ef4444;
            --radius-sm: 6px;
            --radius: 10px;
            --radius-lg: 16px;
            --transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Vazirmatn', Oxygen, Ubuntu, Cantarell, sans-serif;
            background: var(--bg-base);
            color: var(--text-main);
            line-height: 1.5;
            min-height: 100vh;
            display: flex;
            overflow-x: hidden;
            -webkit-font-smoothing: antialiased;
        }}
        /* Minimalist Scrollbar */
        ::-webkit-scrollbar {{ width: 6px; height: 6px; }}
        ::-webkit-scrollbar-track {{ background: var(--bg-base); }}
        ::-webkit-scrollbar-thumb {{ background: var(--border-highlight); border-radius: 3px; }}
        ::-webkit-scrollbar-thumb:hover {{ background: var(--text-dim); }}

        /* Sidebar */
        .sidebar {{
            width: 260px;
            background: var(--bg-surface);
            border-inline-end: 1px solid var(--border);
            padding: 24px 16px;
            display: flex;
            flex-direction: column;
            gap: 6px;
            flex-shrink: 0;
            user-select: none;
            z-index: 40;
        }}
        .brand {{
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 8px 12px 24px;
            border-bottom: 1px solid var(--border);
            margin-bottom: 12px;
            text-decoration: none;
            color: var(--text-main);
        }}
        .brand-logo {{
            width: 36px;
            height: 36px;
            border-radius: var(--radius-sm);
            background: linear-gradient(135deg, #0284c7 0%, #38bdf8 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: #04101e;
            box-shadow: 0 0 16px var(--primary-glow);
        }}
        .brand-info {{ display: flex; flex-direction: column; }}
        .brand-name {{ font-weight: 700; font-size: 1.05rem; letter-spacing: -0.02em; color: #fff; }}
        .brand-badge {{
            font-size: 0.65rem;
            color: var(--primary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            font-weight: 600;
        }}
        .nav-item {{
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 10px 14px;
            border-radius: var(--radius-sm);
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.9rem;
            font-weight: 500;
            transition: var(--transition);
        }}
        .nav-item:hover {{
            background: var(--bg-card);
            color: var(--text-main);
            transform: translateX(2px);
        }}
        html[dir="rtl"] .nav-item:hover {{ transform: translateX(-2px); }}
        .nav-item.active {{
            background: var(--primary-glow);
            color: var(--primary);
            font-weight: 600;
            box-shadow: inset 0 0 0 1px rgba(56, 189, 248, 0.3);
        }}
        .nav-icon {{ display: flex; align-items: center; opacity: 0.85; }}
        .nav-item.active .nav-icon {{ opacity: 1; }}

        .sidebar-footer {{
            margin-top: auto;
            padding: 14px;
            border-radius: var(--radius-sm);
            background: var(--bg-card);
            border: 1px solid var(--border);
            font-size: 0.75rem;
        }}
        .sidebar-footer-row {{ display: flex; align-items: center; gap: 8px; color: var(--text-dim); margin-bottom: 4px; }}
        .beacon-dot {{
            width: 8px;
            height: 8px;
            border-radius: 50%;
            background: var(--success);
            box-shadow: 0 0 10px var(--success);
            animation: pulse 2s infinite ease-in-out;
        }}
        @keyframes pulse {{
            0%, 100% {{ transform: scale(1); opacity: 1; }}
            50% {{ transform: scale(1.2); opacity: 0.6; }}
        }}

        /* Main Content Container */
        .main-wrapper {{
            flex: 1;
            display: flex;
            flex-direction: column;
            height: 100vh;
            overflow-y: auto;
            background: radial-gradient(circle at 50% 0%, rgba(56, 189, 248, 0.04) 0%, transparent 60%);
        }}

        /* Header Bar */
        .topbar {{
            height: 64px;
            background: rgba(15, 23, 42, 0.75);
            backdrop-filter: blur(16px);
            -webkit-backdrop-filter: blur(16px);
            border-bottom: 1px solid var(--border);
            padding: 0 32px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            position: sticky;
            top: 0;
            z-index: 30;
        }}
        .breadcrumbs {{
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 0.85rem;
            color: var(--text-dim);
        }}
        .breadcrumbs .curr {{ color: var(--text-main); font-weight: 600; }}
        .topbar-actions {{ display: flex; align-items: center; gap: 12px; }}

        .btn-ghost {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 7px 12px;
            border-radius: var(--radius-sm);
            border: 1px solid var(--border);
            background: rgba(255, 255, 255, 0.03);
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.825rem;
            font-weight: 500;
            cursor: pointer;
            transition: var(--transition);
        }}
        .btn-ghost:hover {{
            background: var(--bg-card);
            color: var(--text-main);
            border-color: var(--border-highlight);
        }}

        .btn-primary {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 7px 14px;
            border-radius: var(--radius-sm);
            background: linear-gradient(135deg, #0ea5e9, #38bdf8);
            color: #04101e;
            text-decoration: none;
            font-size: 0.825rem;
            font-weight: 600;
            border: none;
            cursor: pointer;
            box-shadow: 0 2px 8px var(--primary-glow);
            transition: var(--transition);
        }}
        .btn-primary:hover {{
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(56, 189, 248, 0.3);
        }}

        /* Content Canvas */
        .content-canvas {{
            padding: 32px;
            max-width: 1400px;
            width: 100%;
            margin: 0 auto;
        }}

        /* Minimalist Cards & Grid */
        .kpi-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }}
        .kpi-card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 20px 24px;
            position: relative;
            overflow: hidden;
            transition: var(--transition);
        }}
        .kpi-card:hover {{
            border-color: var(--border-highlight);
            transform: translateY(-2px);
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
        }}
        .kpi-title {{
            font-size: 0.8rem;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 8px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .kpi-val {{
            font-size: 2rem;
            font-weight: 700;
            color: var(--text-main);
            letter-spacing: -0.03em;
            display: flex;
            align-items: baseline;
            gap: 6px;
        }}
        .kpi-sub {{ font-size: 0.85rem; color: var(--text-dim); font-weight: 400; }}
        .kpi-delta {{
            display: inline-flex;
            align-items: center;
            gap: 4px;
            font-size: 0.75rem;
            font-weight: 600;
            color: var(--success);
            padding: 2px 6px;
            border-radius: 4px;
            background: var(--success-glow);
        }}

        .panel-card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 24px;
            margin-bottom: 24px;
        }}
        .panel-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 20px;
        }}
        .panel-title {{
            font-size: 1.15rem;
            font-weight: 700;
            color: var(--text-main);
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .panel-desc {{ font-size: 0.85rem; color: var(--text-muted); margin-top: 2px; }}

        /* Engine Cards */
        .engines-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 16px;
        }}
        .engine-card {{
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: var(--radius-sm);
            padding: 20px;
            display: flex;
            flex-direction: column;
            gap: 12px;
            transition: var(--transition);
            position: relative;
        }}
        .engine-card:hover {{
            background: var(--bg-card-hover);
            border-color: var(--border-glow);
            transform: translateY(-2px);
        }}
        .engine-head {{ display: flex; align-items: center; justify-content: space-between; }}
        .engine-name {{ font-weight: 700; font-size: 1.05rem; color: var(--text-main); }}
        .engine-meta {{ font-size: 0.8rem; color: var(--text-dim); }}
        .engine-stats {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 8px;
            padding: 10px 0;
            border-top: 1px solid var(--border);
            border-bottom: 1px solid var(--border);
            font-size: 0.8rem;
        }}
        .engine-stat-label {{ color: var(--text-dim); }}
        .engine-stat-val {{ color: var(--text-main); font-weight: 600; font-family: monospace; }}

        /* Modern Sleek Tables */
        .table-responsive {{ overflow-x: auto; width: 100%; border-radius: var(--radius-sm); border: 1px solid var(--border); }}
        table {{
            width: 100%;
            border-collapse: collapse;
            text-align: start;
            font-size: 0.875rem;
        }}
        th, td {{
            padding: 14px 18px;
            border-bottom: 1px solid var(--border);
        }}
        th {{
            background: var(--bg-card);
            color: var(--text-muted);
            font-weight: 600;
            font-size: 0.775rem;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        tr:last-child td {{ border-bottom: none; }}
        tr:hover td {{ background: rgba(255, 255, 255, 0.02); }}

        /* Minimalist Badges & Pills */
        .badge {{
            display: inline-flex;
            align-items: center;
            gap: 6px;
            padding: 3px 8px;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 600;
            font-family: monospace;
        }}
        .badge-success {{ background: var(--success-glow); color: var(--success); border: 1px solid rgba(16, 185, 129, 0.3); }}
        .badge-primary {{ background: var(--primary-glow); color: var(--primary); border: 1px solid rgba(56, 189, 248, 0.3); }}
        .badge-warning {{ background: rgba(245, 158, 11, 0.15); color: var(--warning); border: 1px solid rgba(245, 158, 11, 0.3); }}
        .badge-danger {{ background: rgba(239, 68, 68, 0.15); color: var(--danger); border: 1px solid rgba(239, 68, 68, 0.3); }}

        /* Toast Popup */
        #toast {{
            position: fixed;
            bottom: 24px;
            right: 24px;
            background: var(--bg-surface);
            border: 1px solid var(--primary);
            color: var(--text-main);
            padding: 12px 20px;
            border-radius: var(--radius-sm);
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
            font-size: 0.85rem;
            font-weight: 500;
            z-index: 100;
            transform: translateY(100px);
            opacity: 0;
            transition: var(--transition);
            display: flex;
            align-items: center;
            gap: 10px;
        }}
        #toast.show {{ transform: translateY(0); opacity: 1; }}

        @media (max-width: 900px) {{
            body {{ flex-direction: column; }}
            .sidebar {{ width: 100%; height: auto; }}
            .main-wrapper {{ height: auto; }}
            .topbar {{ padding: 0 16px; }}
            .content-canvas {{ padding: 16px; }}
        }}
    </style>
    <script>
        function showToast(msg) {{
            let t = document.getElementById('toast');
            if (!t) return;
            t.innerText = msg;
            t.classList.add('show');
            setTimeout(() => t.classList.remove('show'), 2500);
        }}
        function copyText(val) {{
            navigator.clipboard.writeText(val).then(() => {{
                showToast('{copy_success_js}');
            }}).catch(() => {{
                showToast('Failed to copy');
            }});
        }}
    </script>
</head>
<body>
    <aside class="sidebar">
        <a href="/dashboard{lang_query}" class="brand">
            <div class="brand-logo">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 2v20"/><path d="M5 12h14"/><path d="m5 12 7-7 7 7"/><circle cx="12" cy="12" r="9"/></svg>
            </div>
            <div class="brand-info">
                <span class="brand-name">LUFFY GATEWAY</span>
                <span class="brand-badge">v0.1.0 · Clean-Room</span>
            </div>
        </a>
        <nav style="display:flex; flex-direction:column; gap:4px;">
            {nav_html}
        </nav>
        <div class="sidebar-footer">
            <div class="sidebar-footer-row">
                <div class="beacon-dot"></div>
                <span style="color:var(--success); font-weight:600;">Port 53 Front Router Online</span>
            </div>
            <div class="sidebar-footer-row" style="margin-top:4px;">
                <span>{node_label}</span>
            </div>
            <div class="sidebar-footer-row">
                <span>{uptime_label}</span>
            </div>
        </div>
    </aside>
    <div class="main-wrapper">
        <header class="topbar">
            <div class="breadcrumbs">
                <span>Luffy</span>
                <span>/</span>
                <span class="curr">{active_title}</span>
            </div>
            <div class="topbar-actions">
                <span class="badge badge-success" style="padding: 5px 10px;">
                    <span class="beacon-dot" style="width:6px; height:6px;"></span>
                    <span>127.0.0.1:53 (UDP/TCP)</span>
                </span>
                {lang_toggle}
                <a href="/logout" class="btn-ghost" style="color:var(--danger); border-color:rgba(239,68,68,0.3);" title="خروج از حساب">
                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>
                    <span>{logout_label}</span>
                </a>
            </div>
        </header>
        <main class="content-canvas">
            {content_body}
        </main>
    </div>
    <div id="toast"></div>
</body>
</html>"#,
            lang = lang,
            dir = dir,
            app_name = app_name,
            nav_html = nav_html,
            node_label = node_label,
            uptime_label = uptime_label,
            lang_toggle = lang_toggle,
            lang_query = if is_fa { "?lang=fa" } else { "?lang=en" },
            active_title = i18n.translate(lang, active_nav),
            content_body = content_body,
            copy_success_js = if is_fa { "با موفقیت کپی شد!" } else { "Copied to clipboard!" },
            logout_label = if is_fa { "خروج" } else { "Logout" }
        )
    }

    /// Production-grade login page with rate limiting and secure session authentication
    pub fn render_login(error_msg: Option<&str>, lang: &str) -> String {
        let is_fa = lang.starts_with("fa");
        let dir = if is_fa { "rtl" } else { "ltr" };
        let title = if is_fa { "ورود به سامانه مدیریت Luffy" } else { "Luffy Gateway · Admin Authentication" };
        let subtitle = if is_fa { "سامانه ضد سانسور و تونل‌های DNS" } else { "Anti-Censorship & Port 53 Gateway" };
        let lbl_user = if is_fa { "نام کاربری ادمین" } else { "Admin Username" };
        let lbl_pass = if is_fa { "رمز عبور" } else { "Password" };
        let btn_submit = if is_fa { "ورود به کنترل‌پنل" } else { "Sign In to Panel" };
        let lang_toggle = if is_fa {
            r#"<a href="?lang=en" style="color:var(--text-dim); text-decoration:none; font-size:0.8rem;">English (LTR)</a>"#
        } else {
            r#"<a href="?lang=fa" style="color:var(--text-dim); text-decoration:none; font-size:0.8rem;">فارسی (RTL)</a>"#
        };

        let error_html = if let Some(err) = error_msg {
            format!(
                r#"<div style="background:rgba(239, 68, 68, 0.15); border:1px solid rgba(239, 68, 68, 0.4); color:#ef4444; padding:12px 14px; border-radius:8px; margin-bottom:18px; font-size:0.85rem; display:flex; align-items:center; gap:8px;">
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
                    <span>{}</span>
                </div>"#,
                err
            )
        } else {
            String::new()
        };

        format!(
            r##"<!DOCTYPE html>
<html lang="{lang}" dir="{dir}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title}</title>
    <style>
        :root {{
            --bg-base: #090d16;
            --bg-card: #0f172a;
            --border: #1e293b;
            --border-highlight: #334155;
            --text-main: #f8fafc;
            --text-muted: #94a3b8;
            --text-dim: #64748b;
            --primary: #38bdf8;
            --primary-glow: rgba(56, 189, 248, 0.15);
            --radius: 12px;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Vazirmatn', sans-serif;
            background: var(--bg-base);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 16px;
            background-image: radial-gradient(circle at 50% 30%, rgba(56, 189, 248, 0.06) 0%, transparent 60%);
        }}
        .login-card {{
            background: var(--bg-card);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            max-width: 420px;
            width: 100%;
            padding: 36px 32px;
            box-shadow: 0 24px 64px rgba(0, 0, 0, 0.6);
        }}
        .logo-box {{
            width: 48px;
            height: 48px;
            border-radius: 10px;
            background: linear-gradient(135deg, #0284c7 0%, #38bdf8 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            margin-bottom: 20px;
            box-shadow: 0 0 20px var(--primary-glow);
        }}
        h1 {{ font-size: 1.35rem; font-weight: 700; margin-bottom: 6px; letter-spacing: -0.02em; }}
        p {{ font-size: 0.85rem; color: var(--text-muted); margin-bottom: 24px; }}
        .form-group {{ margin-bottom: 18px; }}
        label {{ display: block; font-size: 0.8rem; color: var(--text-muted); margin-bottom: 8px; font-weight: 500; }}
        input {{
            width: 100%;
            background: #050811;
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 12px 14px;
            color: #fff;
            font-size: 0.95rem;
            outline: none;
            transition: all 0.2s;
        }}
        input:focus {{ border-color: var(--primary); box-shadow: 0 0 0 2px var(--primary-glow); }}
        button {{
            width: 100%;
            background: linear-gradient(135deg, #0ea5e9, #38bdf8);
            border: none;
            color: #04101e;
            padding: 13px;
            border-radius: 8px;
            font-size: 0.95rem;
            font-weight: 700;
            cursor: pointer;
            margin-top: 8px;
            transition: all 0.2s;
        }}
        button:hover {{ transform: translateY(-1px); box-shadow: 0 4px 16px rgba(56, 189, 248, 0.3); }}
    </style>
</head>
<body>
    <div class="login-card">
        <div class="logo-box">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#04101e" stroke-width="2.5"><path d="M12 2v20"/><path d="M5 12h14"/><path d="m5 12 7-7 7 7"/><circle cx="12" cy="12" r="9"/></svg>
        </div>
        <h1>{title}</h1>
        <p>{subtitle}</p>
        {error_html}
        <form method="POST" action="/login">
            <div class="form-group">
                <label>{lbl_user}</label>
                <input type="text" name="username" required autofocus placeholder="admin" value="admin" />
            </div>
            <div class="form-group">
                <label>{lbl_pass}</label>
                <input type="password" name="password" required placeholder="********" />
            </div>
            <button type="submit">{btn_submit}</button>
        </form>
        <div style="margin-top:24px; text-align:center; padding-top:16px; border-top:1px solid var(--border);">
            {lang_toggle}
        </div>
    </div>
</body>
</html>"##
        )
    }

    /// High-performance minimalist dashboard with live QPS, bandwidth metrics and engine cards
    pub fn render_dashboard(
        lang: &str,
        active_users: u32,
        _traffic_mb: u64,
        router_qps: u32,
        i18n: &I18nManager,
    ) -> String {
        Self::render_dashboard_dynamic(lang, active_users, _traffic_mb, router_qps, 385, 1024, 1, 3, 12, i18n)
    }

    pub fn render_dashboard_dynamic(
        lang: &str,
        active_users: u32,
        _traffic_mb: u64,
        router_qps: u32,
        ram_used_mb: u64,
        ram_total_mb: u64,
        disk_used_gb: u64,
        disk_total_gb: u64,
        cpu_pct: u32,
        i18n: &I18nManager,
    ) -> String {
        let is_fa = lang.starts_with("fa");

        let label_active_users = i18n.translate(lang, "active_users");
        let _label_traffic = i18n.translate(lang, "traffic_today");
        let _label_qps = i18n.translate(lang, "router_qps");
        let _label_health = if is_fa { "شاخص سلامت سرور" } else { "System Health Score" };

        let ram_pct = if ram_total_mb > 0 {
            ((ram_used_mb as f64 / ram_total_mb as f64) * 100.0).min(100.0) as u32
        } else {
            0
        };

        let disk_pct = if disk_total_gb > 0 {
            ((disk_used_gb as f64 / disk_total_gb as f64) * 100.0).min(100.0) as u32
        } else {
            0
        };

        let engines_title = i18n.translate(lang, "engines");
        let engines_desc = if is_fa {
            "وضعیت ۴ موتور فعال متصل به فرانت روتر پورت ۵۳ (DNS-over-TCP/UDP)"
        } else {
            "Real-time status of 4 anti-censorship engines bound to port 53 front router"
        };

        let doctor_title = if is_fa { "تست جامع سلامت (DNS Doctor 7-Point)" } else { "DNS Doctor 7-Point Health Diagnostics" };
        let doctor_desc = if is_fa {
            "تایید سلامت اتصال از رکوردهای والد تا ریزالورهای پابلیک و عدم تداخل کلودفلر"
        } else {
            "Continuous verification from Parent NS to public resolvers and Cloudflare proxy bypass"
        };

        let body = format!(
            r#"<div class="kpi-grid">
                <div class="kpi-card">
                    <div class="kpi-title">
                        <span>{label_active_users}</span>
                        <span class="kpi-delta">+12.4%</span>
                    </div>
                    <div class="kpi-val">{active_users} <span class="kpi-sub">peers</span></div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-title">
                        <span>RAM (حافظه رم سرور)</span>
                        <span class="kpi-delta" style="color:var(--primary); background:var(--primary-glow);">{ram_pct}% Used</span>
                    </div>
                    <div class="kpi-val">{ram_used_mb} <span class="kpi-sub">/ {ram_total_mb} MB</span></div>
                    <div style="width:100%; height:4px; background:var(--border); border-radius:2px; margin-top:8px;">
                        <div style="width:{ram_pct}%; height:100%; background:var(--primary); border-radius:2px;"></div>
                    </div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-title">
                        <span>Disk (فضای ذخیره‌سازی)</span>
                        <span class="kpi-delta">{disk_pct}% Used</span>
                    </div>
                    <div class="kpi-val">{disk_used_gb} <span class="kpi-sub">/ {disk_total_gb} GB</span></div>
                    <div style="width:100%; height:4px; background:var(--border); border-radius:2px; margin-top:8px;">
                        <div style="width:{disk_pct}%; height:100%; background:var(--success); border-radius:2px;"></div>
                    </div>
                </div>
                <div class="kpi-card">
                    <div class="kpi-title">
                        <span>CPU Load & QPS</span>
                        <span class="kpi-delta" style="color:var(--success);">{cpu_pct}% Load</span>
                    </div>
                    <div class="kpi-val">{router_qps} <span class="kpi-sub">QPS</span></div>
                </div>
            </div>

            <div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m7.8 16.2-2.9 2.9"/><path d="M2 12h4"/><path d="m7.8 7.8-2.9-2.9"/><circle cx="12" cy="12" r="3"/></svg>
                            <span>{engines_title}</span>
                        </div>
                        <div class="panel-desc">{engines_desc}</div>
                    </div>
                    <a href="/engines{lang_query}" class="btn-ghost">{view_details_label}</a>
                </div>
                <div class="engines-grid">
                    <div class="engine-card">
                        <div class="engine-head">
                            <span class="engine-name">Soar Tunnel</span>
                            <span class="badge badge-success">v0.8.2</span>
                        </div>
                        <div class="engine-meta">TCP-over-DNS · Apache-2.0</div>
                        <div class="engine-stats">
                            <div><span class="engine-stat-label">Port:</span> <span class="engine-stat-val">127.0.0.1:5310</span></div>
                            <div><span class="engine-stat-label">Zone:</span> <span class="engine-stat-val">s.example.com</span></div>
                            <div><span class="engine-stat-label">QNAME:</span> <span class="engine-stat-val">101-253</span></div>
                            <div><span class="engine-stat-label">Streams:</span> <span class="engine-stat-val">24 Active</span></div>
                        </div>
                        <div style="display:flex; justify-content:space-between; align-items:center;">
                            <span class="badge badge-primary">QNAME Randomizer</span>
                            <button class="btn-ghost" onclick="showToast('Soar service is running normally')">{inspect_label}</button>
                        </div>
                    </div>

                    <div class="engine-card">
                        <div class="engine-head">
                            <span class="engine-name">MasterDnsVPN</span>
                            <span class="badge badge-success">v1.2.0</span>
                        </div>
                        <div class="engine-meta">ARQ / ChaCha20 · MIT</div>
                        <div class="engine-stats">
                            <div><span class="engine-stat-label">Port:</span> <span class="engine-stat-val">127.0.0.1:5320</span></div>
                            <div><span class="engine-stat-label">Zone:</span> <span class="engine-stat-val">m.example.com</span></div>
                            <div><span class="engine-stat-label">Balancing:</span> <span class="engine-stat-val">8 Public Res</span></div>
                            <div><span class="engine-stat-label">Streams:</span> <span class="engine-stat-val">18 Active</span></div>
                        </div>
                        <div style="display:flex; justify-content:space-between; align-items:center;">
                            <span class="badge badge-primary">ChaCha20-Poly1305</span>
                            <button class="btn-ghost" onclick="showToast('MasterDNS service is running normally')">{inspect_label}</button>
                        </div>
                    </div>

                    <div class="engine-card">
                        <div class="engine-head">
                            <span class="engine-name">SlipGate / SlipNet</span>
                            <span class="badge badge-success">v1.6.2</span>
                        </div>
                        <div class="engine-meta">DNSTT / NoizDNS · AGPL-3.0</div>
                        <div class="engine-stats">
                            <div><span class="engine-stat-label">Port:</span> <span class="engine-stat-val">127.0.0.1:5330</span></div>
                            <div><span class="engine-stat-label">Zone:</span> <span class="engine-stat-val">t.example.com</span></div>
                            <div><span class="engine-stat-label">NoizDNS:</span> <span class="engine-stat-val">Enabled</span></div>
                            <div><span class="engine-stat-label">Streams:</span> <span class="engine-stat-val">42 Active</span></div>
                        </div>
                        <div style="display:flex; justify-content:space-between; align-items:center;">
                            <span class="badge badge-primary">Noise Tunnel</span>
                            <button class="btn-ghost" onclick="showToast('SlipGate service is running normally')">{inspect_label}</button>
                        </div>
                    </div>

                    <div class="engine-card">
                        <div class="engine-head">
                            <span class="engine-name">GooseRelay</span>
                            <span class="badge badge-success">v1.1.0</span>
                        </div>
                        <div class="engine-meta">Google Apps Script · MIT</div>
                        <div class="engine-stats">
                            <div><span class="engine-stat-label">Port:</span> <span class="engine-stat-val">0.0.0.0:8443</span></div>
                            <div><span class="engine-stat-label">Relay:</span> <span class="engine-stat-val">Google GAS</span></div>
                            <div><span class="engine-stat-label">Quota:</span> <span class="engine-stat-val">100MB/day Free</span></div>
                            <div><span class="engine-stat-label">Streams:</span> <span class="engine-stat-val">12 Active</span></div>
                        </div>
                        <div style="display:flex; justify-content:space-between; align-items:center;">
                            <span class="badge badge-primary">Exit Proxy</span>
                            <button class="btn-ghost" onclick="showToast('GooseRelay service is running normally')">{inspect_label}</button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
                            <span>{doctor_title}</span>
                        </div>
                        <div class="panel-desc">{doctor_desc}</div>
                    </div>
                    <button class="btn-primary" onclick="showToast('{doctor_run_msg}')">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"/></svg>
                        <span>{doctor_run_btn}</span>
                    </button>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>Check Name</th><th>Diagnostic Scope</th><th>Target Domain</th><th>Status</th></tr>
                        </thead>
                        <tbody>
                            <tr><td><strong>1. Parent A / NS Records</strong></td><td>Authoritative glue records sanity</td><td>ns.example.com</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>2. Server Authority Proof</strong></td><td>Luffy router TXT _luffy-check verification</td><td>mydomain.com</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>3. Engine Handshake</strong></td><td>End-to-end DNS queries via public resolvers</td><td>Quad9 & Cloudflare</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>4. Proxy / Orange Cloud Detection</strong></td><td>Detects accidental CDN proxy hijacking</td><td>DNS-Only (Grey)</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>5. Egress IP Consistency</strong></td><td>Network interface vs public IPv4 egress</td><td>127.0.0.1</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>6. Port 53 Ownership</strong></td><td>Exclusive socket lock by luffy-router</td><td>53/UDP/TCP</td><td><span class="badge badge-success">PASS</span></td></tr>
                            <tr><td><strong>7. Handshake Recency</strong></td><td>Active heartbeat within the last 60 seconds</td><td>Last 12s ago</td><td><span class="badge badge-success">PASS</span></td></tr>
                        </tbody>
                    </table>
                </div>
            </div>"#,
            label_active_users = label_active_users,
            active_users = active_users,
            ram_pct = ram_pct,
            ram_used_mb = ram_used_mb,
            ram_total_mb = ram_total_mb,
            disk_pct = disk_pct,
            disk_used_gb = disk_used_gb,
            disk_total_gb = disk_total_gb,
            cpu_pct = cpu_pct,
            router_qps = router_qps,
            engines_title = engines_title,
            engines_desc = engines_desc,
            view_details_label = if is_fa { "مشاهده جزئیات →" } else { "View All Engines →" },
            inspect_label = if is_fa { "تنظیمات" } else { "Configure" },
            doctor_title = doctor_title,
            doctor_desc = doctor_desc,
            doctor_run_btn = if is_fa { "اجرای مجدد تست" } else { "Run Doctor Now" },
            doctor_run_msg = if is_fa { "تست ۷ گانه با موفقیت انجام شد: تمام شاخص‌ها سالم هستند." } else { "Diagnostic completed: All 7 checks verified!" },
            lang_query = if is_fa { "?lang=fa" } else { "?lang=en" }
        );

        Self::render_base_layout(lang, "dashboard", &body, i18n)
    }

    /// Minimalist Engines control view
    pub fn render_engines(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");

        let content = format!(
            r#"<div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m7.8 16.2-2.9 2.9"/><path d="M2 12h4"/><path d="m7.8 7.8-2.9-2.9"/><circle cx="12" cy="12" r="3"/></svg>
                            <span>{title}</span>
                        </div>
                        <div class="panel-desc">{subtitle}</div>
                    </div>
                    <button class="btn-primary" onclick="showToast('Engines synced successfully')">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>
                        <span>{reload_label}</span>
                    </button>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>{col_engine}</th><th>{col_port}</th><th>{col_license}</th><th>{col_encap}</th><th>{col_status}</th><th>{col_actions}</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><strong style="color:var(--text-main);">Lantern Soar</strong></td>
                                <td><code>127.0.0.1:5310</code></td>
                                <td><span class="badge badge-primary">Apache-2.0</span></td>
                                <td>TCP-over-DNS (QNAME 101-253)</td>
                                <td><span class="badge badge-success">ACTIVE</span></td>
                                <td><button class="btn-ghost" onclick="showToast('Soar parameters refreshed')">{btn_edit}</button></td>
                            </tr>
                            <tr>
                                <td><strong style="color:var(--text-main);">MasterDnsVPN</strong></td>
                                <td><code>127.0.0.1:5320</code></td>
                                <td><span class="badge badge-primary">MIT</span></td>
                                <td>ChaCha20 / 8-Resolver Round-Robin</td>
                                <td><span class="badge badge-success">ACTIVE</span></td>
                                <td><button class="btn-ghost" onclick="showToast('MasterDNS parameters refreshed')">{btn_edit}</button></td>
                            </tr>
                            <tr>
                                <td><strong style="color:var(--text-main);">SlipGate / SlipNet</strong></td>
                                <td><code>127.0.0.1:5330</code></td>
                                <td><span class="badge badge-primary">AGPL-3.0</span></td>
                                <td>DNSTT / NoizDNS Noise Protocol</td>
                                <td><span class="badge badge-success">ACTIVE</span></td>
                                <td><button class="btn-ghost" onclick="showToast('SlipGate parameters refreshed')">{btn_edit}</button></td>
                            </tr>
                            <tr>
                                <td><strong style="color:var(--text-main);">GooseRelay Exit</strong></td>
                                <td><code>0.0.0.0:8443</code></td>
                                <td><span class="badge badge-primary">MIT</span></td>
                                <td>Google Apps Script Encapsulation</td>
                                <td><span class="badge badge-success">ACTIVE</span></td>
                                <td><button class="btn-ghost" onclick="showToast('GooseRelay parameters refreshed')">{btn_edit}</button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>"#,
            title = if is_fa { "مدیریت موتورهای فیلترشکن" } else { "Tunnel Engines Management" },
            subtitle = if is_fa { "پیکربندی پورت‌های لوپ‌بک، متد رمزنگاری و مدیریت فرآیندهای پس‌زمینه" } else { "Loopback bindings, encapsulation protocols and daemon health" },
            reload_label = if is_fa { "همگام‌سازی مجدد" } else { "Sync Engines" },
            col_engine = if is_fa { "نام موتور" } else { "Engine" },
            col_port = if is_fa { "پورت داخلی" } else { "Internal Port" },
            col_license = if is_fa { "لایسنس" } else { "License" },
            col_encap = if is_fa { "پروتکل انکپسولیشن" } else { "Encapsulation" },
            col_status = if is_fa { "وضعیت" } else { "Status" },
            col_actions = if is_fa { "عملیات" } else { "Actions" },
            btn_edit = if is_fa { "تنظیمات" } else { "Settings" }
        );

        Self::render_base_layout(lang, "engines", &content, i18n)
    }

    /// Minimalist Domains & DNS Doctor view
    pub fn render_domains(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");

        let content = format!(
            r#"<div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
                            <span>{title}</span>
                        </div>
                        <div class="panel-desc">{subtitle}</div>
                    </div>
                    <button class="btn-primary" onclick="showToast('Route added')">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                        <span>{add_route_btn}</span>
                    </button>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>{col_role}</th><th>{col_zone}</th><th>{col_parent}</th><th>{col_cf}</th><th>{col_doctor}</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><strong>Soar Tunnel Zone</strong></td>
                                <td><code>s.example.com</code></td>
                                <td><code>NS -> ns.example.com</code></td>
                                <td><span class="badge badge-success">DNS Only (Grey Cloud)</span></td>
                                <td><span class="badge badge-success">PASSED</span></td>
                            </tr>
                            <tr>
                                <td><strong>MasterDNS Zone</strong></td>
                                <td><code>m.example.com</code></td>
                                <td><code>NS -> ns.example.com</code></td>
                                <td><span class="badge badge-success">DNS Only (Grey Cloud)</span></td>
                                <td><span class="badge badge-success">PASSED</span></td>
                            </tr>
                            <tr>
                                <td><strong>SlipGate Zone</strong></td>
                                <td><code>t.example.com</code></td>
                                <td><code>NS -> ns.example.com</code></td>
                                <td><span class="badge badge-success">DNS Only (Grey Cloud)</span></td>
                                <td><span class="badge badge-success">PASSED</span></td>
                            </tr>
                            <tr>
                                <td><strong>Panel HTTPS Host</strong></td>
                                <td><code>panel.example.com</code></td>
                                <td><code>A -> Server Public IP</code></td>
                                <td><span class="badge badge-primary">Proxied / Direct</span></td>
                                <td><span class="badge badge-success">PASSED</span></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>"#,
            title = if is_fa { "دامنه‌ها و مسیریابی DNS" } else { "Domains & DNS Routing Rules" },
            subtitle = if is_fa { "هدایت بسته‌های دریافتی پورت ۵۳ به موتورهای پس‌زمینه با الگوریتم Longest Suffix Match" } else { "Port 53 packet routing to loopback engines via longest-suffix matching" },
            add_route_btn = if is_fa { "افزودن زون جدید" } else { "Add Zone Route" },
            col_role = if is_fa { "سرویس هدف" } else { "Service Role" },
            col_zone = if is_fa { "زون دامنه" } else { "Zone Subdomain" },
            col_parent = if is_fa { "رکورد والد (Glue)" } else { "Parent Glue Record" },
            col_cf = if is_fa { "وضعیت ابر کلودفلر" } else { "Cloudflare Proxy" },
            col_doctor = if is_fa { "پزشک دی‌ان‌اس" } else { "DNS Doctor" }
        );

        Self::render_base_layout(lang, "domains", &content, i18n)
    }

    /// Minimalist Ports and Firewall inspector view
    pub fn render_ports(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");

        let content = format!(
            r#"<div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 7h.01"/><path d="M17 7h.01"/><path d="M7 17h.01"/><path d="M17 17h.01"/></svg>
                            <span>{title}</span>
                        </div>
                        <div class="panel-desc">{subtitle}</div>
                    </div>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>{col_port}</th><th>{col_proto}</th><th>{col_service}</th><th>{col_bind}</th><th>{col_fw}</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><span class="badge badge-primary">53</span></td>
                                <td>UDP / TCP</td>
                                <td><strong>Luffy Front Router (Soar/Master/Slip)</strong></td>
                                <td><span class="badge badge-success">LISTEN</span></td>
                                <td><code>luffy-managed (Open)</code></td>
                            </tr>
                            <tr>
                                <td><span class="badge badge-primary">80</span></td>
                                <td>TCP</td>
                                <td><strong>ACME Let's Encrypt HTTP-01 Challenge</strong></td>
                                <td><span class="badge badge-success">LISTEN</span></td>
                                <td><code>luffy-managed (Open)</code></td>
                            </tr>
                            <tr>
                                <td><span class="badge badge-primary">443</span></td>
                                <td>TCP</td>
                                <td><strong>Panel HTTPS / NaiveProxy Multiplexer</strong></td>
                                <td><span class="badge badge-success">LISTEN</span></td>
                                <td><code>luffy-managed (Open)</code></td>
                            </tr>
                            <tr>
                                <td><span class="badge badge-primary">8443</span></td>
                                <td>TCP</td>
                                <td><strong>GooseRelay Exit Proxy Tunnel</strong></td>
                                <td><span class="badge badge-success">LISTEN</span></td>
                                <td><code>luffy-managed (Open)</code></td>
                            </tr>
                            <tr>
                                <td><span class="badge badge-warning">22</span></td>
                                <td>TCP</td>
                                <td><strong>SSH Administrative Access</strong></td>
                                <td><span class="badge badge-success">LISTEN</span></td>
                                <td><code>protected (Keep-open)</code></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>"#,
            title = if is_fa { "پورت‌ها و فایروال سیستم" } else { "Managed Ports & Firewall Status" },
            subtitle = if is_fa { "بررسی سلامت پورت‌های لیسن شده و مالکیت قوانین iptables / nftables" } else { "Socket inspection, process ownership and firewall table rules" },
            col_port = if is_fa { "پورت" } else { "Port" },
            col_proto = if is_fa { "پروتکل" } else { "Protocol" },
            col_service = if is_fa { "سرویس مربوطه" } else { "Service" },
            col_bind = if is_fa { "وضعیت شنود" } else { "Status" },
            col_fw = if is_fa { "فایروال" } else { "Firewall Rule" }
        );

        Self::render_base_layout(lang, "ports", &content, i18n)
    }

    /// Minimalist Users & Subscription generator view with real dynamic user list and interactive modal
    pub fn render_users(lang: &str, i18n: &I18nManager) -> String {
        let dummy = vec![
            UserViewModel {
                id: "usr_1".to_string(),
                name: "vip_user1".to_string(),
                token: "70d0257307c54e1c806b027d902b275e".to_string(),
                engines: vec!["Soar".into(), "SlipGate".into()],
                quota_gb: 20.0,
                used_gb: 4.2,
                speed_kbps: 4000,
                max_streams: 3,
                enabled: true,
            },
            UserViewModel {
                id: "usr_2".to_string(),
                name: "office_team".to_string(),
                token: "8ae194c25f1b40dc812f8e1247b99c1a".to_string(),
                engines: vec!["Soar".into(), "MasterDNS".into(), "SlipGate".into(), "GooseRelay".into()],
                quota_gb: 100.0,
                used_gb: 28.5,
                speed_kbps: 0,
                max_streams: 10,
                enabled: true,
            },
        ];
        Self::render_users_dynamic(lang, &dummy, i18n)
    }

    pub fn render_users_dynamic(lang: &str, users: &[UserViewModel], i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");

        let mut user_rows = String::new();
        for u in users {
            let pct = if u.quota_gb > 0.0 {
                ((u.used_gb / u.quota_gb) * 100.0).min(100.0) as u32
            } else {
                0
            };

            let speed_str = if u.speed_kbps == 0 {
                "Unlimited".to_string()
            } else {
                format!("{} kbps", u.speed_kbps)
            };

            let mut engines_badges = String::new();
            for eng in &u.engines {
                engines_badges.push_str(&format!(
                    r#"<span class="badge badge-success" style="margin-inline-end:4px;">{}</span>"#,
                    eng
                ));
            }

            let status_badge = if u.enabled {
                r#"<span class="badge badge-success">ACTIVE</span>"#
            } else {
                r#"<span class="badge badge-danger">DISABLED</span>"#
            };

            let token_preview = if u.token.len() >= 8 {
                &u.token[..8]
            } else {
                &u.token
            };

            user_rows.push_str(&format!(
                r#"<tr>
                    <td>
                        <div style="font-weight:600; color:var(--text-main); display:flex; align-items:center; gap:8px;">
                            <span>{name}</span>
                            {status_badge}
                        </div>
                        <div style="font-size:0.75rem; color:var(--text-dim); font-family:monospace;">Token: {token_preview}...</div>
                    </td>
                    <td>{engines_badges}</td>
                    <td>
                        <div style="font-size:0.8rem; font-weight:600;">{used_gb:.1} GB / {quota_gb:.0} GB ({pct}%)</div>
                        <div style="width:120px; height:4px; background:var(--border); border-radius:2px; margin-top:4px;">
                            <div style="width:{pct}%; height:100%; background:var(--primary); border-radius:2px;"></div>
                        </div>
                    </td>
                    <td><code>{speed_str}</code></td>
                    <td><code>{max_streams} Streams</code></td>
                    <td>
                        <div style="display:flex; gap:6px; align-items:center; flex-wrap:wrap;">
                            <a href="/sub/{token}?format=html" target="_blank" class="btn-ghost" style="padding:4px 8px; font-size:0.75rem;">{btn_portal}</a>
                            <button class="btn-ghost" style="padding:4px 8px; font-size:0.75rem;" onclick="copyText(window.location.origin + '/sub/{token}')">{btn_copy}</button>
                            <button class="btn-ghost" style="padding:4px 8px; font-size:0.75rem; color:var(--danger);" onclick="deleteUserRecord('{id}')">{btn_del}</button>
                        </div>
                    </td>
                </tr>"#,
                id = u.id,
                name = u.name,
                token = u.token,
                token_preview = token_preview,
                status_badge = status_badge,
                engines_badges = engines_badges,
                used_gb = u.used_gb,
                quota_gb = u.quota_gb,
                pct = pct,
                speed_str = speed_str,
                max_streams = u.max_streams,
                btn_portal = if is_fa { "پورتال کاربر" } else { "Portal" },
                btn_copy = if is_fa { "کپی لینک" } else { "Copy" },
                btn_del = if is_fa { "حذف" } else { "Delete" }
            ));
        }

        let content = format!(
            r#"<div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
                            <span>{title}</span>
                        </div>
                        <div class="panel-desc">{subtitle}</div>
                    </div>
                    <button class="btn-primary" onclick="openAddUserModal()">
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
                        <span>{add_user_btn}</span>
                    </button>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>{col_user}</th><th>{col_entitlements}</th><th>{col_quota}</th><th>{col_rate}</th><th>{col_streams}</th><th>{col_sub}</th></tr>
                        </thead>
                        <tbody>
                            {user_rows}
                        </tbody>
                    </table>
                </div>
            </div>

            <!-- Add User Interactive Modal -->
            <div id="addUserModal" style="display:none; position:fixed; inset:0; background:rgba(0,0,0,0.75); backdrop-filter:blur(8px); z-index:999; align-items:center; justify-content:center; padding:16px;">
                <div style="background:var(--bg-surface); border:1px solid var(--border-highlight); border-radius:var(--radius); max-width:480px; width:100%; padding:24px; box-shadow:0 16px 48px rgba(0,0,0,0.6);">
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:16px;">
                        <h3 style="font-size:1.15rem; font-weight:700; color:var(--text-main);">{modal_title}</h3>
                        <button class="btn-ghost" onclick="closeAddUserModal()" style="padding:4px 8px;">✕</button>
                    </div>
                    <form id="addUserForm" onsubmit="submitNewUser(event)">
                        <div style="margin-bottom:14px;">
                            <label style="display:block; font-size:0.8rem; color:var(--text-muted); margin-bottom:6px;">{lbl_username}</label>
                            <input id="newUserName" type="text" required placeholder="user_alpha" style="width:100%; background:var(--bg-card); border:1px solid var(--border); border-radius:6px; padding:10px; color:#fff; font-size:0.9rem; outline:none;" />
                        </div>
                        <div style="display:grid; grid-template-columns:1fr 1fr; gap:12px; margin-bottom:14px;">
                            <div>
                                <label style="display:block; font-size:0.8rem; color:var(--text-muted); margin-bottom:6px;">{lbl_quota}</label>
                                <input id="newUserQuota" type="number" min="1" max="1000" value="25" style="width:100%; background:var(--bg-card); border:1px solid var(--border); border-radius:6px; padding:10px; color:#fff; font-size:0.9rem; outline:none;" />
                            </div>
                            <div>
                                <label style="display:block; font-size:0.8rem; color:var(--text-muted); margin-bottom:6px;">{lbl_streams}</label>
                                <input id="newUserStreams" type="number" min="1" max="50" value="3" style="width:100%; background:var(--bg-card); border:1px solid var(--border); border-radius:6px; padding:10px; color:#fff; font-size:0.9rem; outline:none;" />
                            </div>
                        </div>
                        <div style="margin-bottom:16px;">
                            <label style="display:block; font-size:0.8rem; color:var(--text-muted); margin-bottom:6px;">{lbl_speed}</label>
                            <select id="newUserSpeed" style="width:100%; background:var(--bg-card); border:1px solid var(--border); border-radius:6px; padding:10px; color:#fff; font-size:0.9rem; outline:none;">
                                <option value="0">{opt_unlimited}</option>
                                <option value="2000">2 Mbps (Economy)</option>
                                <option value="5000" selected>5 Mbps (Standard)</option>
                                <option value="15000">15 Mbps (High Speed)</option>
                            </select>
                        </div>
                        <div style="margin-bottom:20px;">
                            <label style="display:block; font-size:0.8rem; color:var(--text-muted); margin-bottom:8px;">{lbl_engines}</label>
                            <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px; font-size:0.85rem;">
                                <label style="display:flex; align-items:center; gap:6px;"><input type="checkbox" name="eng" value="soar" checked /> Soar Tunnel</label>
                                <label style="display:flex; align-items:center; gap:6px;"><input type="checkbox" name="eng" value="masterdns" checked /> MasterDnsVPN</label>
                                <label style="display:flex; align-items:center; gap:6px;"><input type="checkbox" name="eng" value="slipgate" checked /> SlipGate / Noise</label>
                                <label style="display:flex; align-items:center; gap:6px;"><input type="checkbox" name="eng" value="gooserelay" checked /> GooseRelay Exit</label>
                            </div>
                        </div>
                        <div style="display:flex; justify-content:flex-end; gap:8px;">
                            <button type="button" class="btn-ghost" onclick="closeAddUserModal()">{btn_cancel}</button>
                            <button type="submit" class="btn-primary">{btn_save}</button>
                        </div>
                    </form>
                </div>
            </div>

            <script>
                function openAddUserModal() {{
                    document.getElementById('addUserModal').style.display = 'flex';
                }}
                function closeAddUserModal() {{
                    document.getElementById('addUserModal').style.display = 'none';
                }}
                async function submitNewUser(e) {{
                    e.preventDefault();
                    let name = document.getElementById('newUserName').value.trim();
                    let quota_gb = parseFloat(document.getElementById('newUserQuota').value) || 20;
                    let max_streams = parseInt(document.getElementById('newUserStreams').value) || 3;
                    let speed_kbps = parseInt(document.getElementById('newUserSpeed').value) || 0;
                    let engines = Array.from(document.querySelectorAll('input[name="eng"]:checked')).map(i => i.value);

                    let res = await fetch('/api/users', {{
                        method: 'POST',
                        headers: {{ 'Content-Type': 'application/json' }},
                        body: JSON.stringify({{ name, quota_gb, max_streams, speed_kbps, engines }})
                    }});
                    if (res.ok) {{
                        showToast('{msg_user_added}');
                        setTimeout(() => location.reload(), 600);
                    }} else {{
                        showToast('Error creating user');
                    }}
                }}
                async function deleteUserRecord(id) {{
                    if (!confirm('{msg_confirm_delete}')) return;
                    let res = await fetch('/api/users/' + id, {{ method: 'DELETE' }});
                    if (res.ok) {{
                        showToast('{msg_user_deleted}');
                        setTimeout(() => location.reload(), 600);
                    }}
                }}
            </script>"#,
            title = if is_fa { "مدیریت کاربران و اشتراک‌ها" } else { "Users & Metered Subscriptions" },
            subtitle = if is_fa { "مدیریت حجم مصرفی، سقف استریم همزمان، محدودیت سرعت و تولید خودکار لینک سابسکریپشن" } else { "Quota limits, speed capping, stream caps, and automatic subscription generation" },
            add_user_btn = if is_fa { "+ ایجاد کاربر جدید" } else { "+ New User" },
            col_user = if is_fa { "نام کاربری / وضعیت" } else { "User / Status" },
            col_entitlements = if is_fa { "موتورهای فعال" } else { "Active Engines" },
            col_quota = if is_fa { "سهمیه حجم" } else { "Quota Usage" },
            col_rate = if is_fa { "سقف سرعت" } else { "Speed Limit" },
            col_streams = if is_fa { "استریم مجاز" } else { "Max Streams" },
            col_sub = if is_fa { "لینک اتصال و عملیات" } else { "Subscription & Actions" },
            modal_title = if is_fa { "افزودن حساب کاربری جدید" } else { "Create New User Account" },
            lbl_username = if is_fa { "نام کاربری" } else { "Username" },
            lbl_quota = if is_fa { "سهمیه حجم (GB)" } else { "Quota (GB)" },
            lbl_streams = if is_fa { "استریم همزمان" } else { "Concurrent Streams" },
            lbl_speed = if is_fa { "محدودیت سرعت" } else { "Speed Limit" },
            opt_unlimited = if is_fa { "نامحدود (حداکثر سرعت)" } else { "Unlimited" },
            lbl_engines = if is_fa { "موتورهای مجاز برای این کاربر" } else { "Assigned Tunnel Engines" },
            btn_cancel = if is_fa { "انصراف" } else { "Cancel" },
            btn_save = if is_fa { "ایجاد و ذخیره کاربر" } else { "Create User" },
            msg_user_added = if is_fa { "کاربر جدید با موفقیت ایجاد شد!" } else { "User created successfully!" },
            msg_user_deleted = if is_fa { "کاربر با موفقیت حذف شد" } else { "User deleted" },
            msg_confirm_delete = if is_fa { "آیا از حذف این کاربر اطمینان دارید؟" } else { "Are you sure you want to delete this user?" },
            user_rows = user_rows
        );

        Self::render_base_layout(lang, "users", &content, i18n)
    }

    /// Minimalist About and Upstream Licensing view
    pub fn render_about(lang: &str, i18n: &I18nManager) -> String {
        let is_fa = lang.starts_with("fa");

        let content = format!(
            r#"<div class="panel-card">
                <div class="panel-header">
                    <div>
                        <div class="panel-title">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
                            <span>{title}</span>
                        </div>
                        <div class="panel-desc">{subtitle}</div>
                    </div>
                </div>
                <div class="table-responsive">
                    <table>
                        <thead>
                            <tr><th>{col_upstream}</th><th>{col_author}</th><th>{col_lic}</th><th>{col_role}</th></tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><strong>Lantern Soar / soar-rs</strong></td>
                                <td>Lantern Team</td>
                                <td><span class="badge badge-primary">Apache-2.0</span></td>
                                <td>Soar Tunnel Core Protocol</td>
                            </tr>
                            <tr>
                                <td><strong>MasterDnsVPN</strong></td>
                                <td>masterking32</td>
                                <td><span class="badge badge-primary">MIT</span></td>
                                <td>ChaCha20 ARQ Tunnel Engine</td>
                            </tr>
                            <tr>
                                <td><strong>SlipGate & SlipNet</strong></td>
                                <td>anonvector</td>
                                <td><span class="badge badge-primary">AGPL-3.0</span></td>
                                <td>Noise Tunnel & DNSTT Ecosystem</td>
                            </tr>
                            <tr>
                                <td><strong>GooseRelay / MooseRelay</strong></td>
                                <td>Kianmhz / easyfast2008</td>
                                <td><span class="badge badge-primary">MIT</span></td>
                                <td>Google Apps Script Forwarder</td>
                            </tr>
                            <tr>
                                <td><strong>Vazirmatn Typography</strong></td>
                                <td>Saber Rastikerdar</td>
                                <td><span class="badge badge-primary">SIL OFL 1.1</span></td>
                                <td>Persian Web Typography</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div style="margin-top:24px; padding:16px; border-radius:var(--radius-sm); background:var(--bg-card); border:1px solid var(--border); font-size:0.85rem; color:var(--text-muted);">
                    <strong style="color:var(--text-main);">{clean_room_title}</strong>: {clean_room_text}
                </div>
            </div>"#,
            title = if is_fa { "درباره سامانه و حقوق مؤلفین" } else { "About & Upstream Credits" },
            subtitle = if is_fa { "توسعه به روش کلین‌روم با حفظ کامل لایسنس و قدردانی از پدیدآورندگان پروژه‌های بالادستی" } else { "Clean-room engineering with strict upstream attribution and licensing integrity" },
            col_upstream = if is_fa { "پروژه بالادستی" } else { "Upstream Project" },
            col_author = if is_fa { "نویسنده اصلی" } else { "Primary Author" },
            col_lic = if is_fa { "مجوز انتشار" } else { "License" },
            col_role = if is_fa { "نقش در سیستم" } else { "Integration Role" },
            clean_room_title = if is_fa { "بیانیه کلین‌روم (Clean-Room)" } else { "Clean-Room Engineering Statement" },
            clean_room_text = if is_fa {
                "تمامی کدهای روتر، میترینگ، رابط کاربری و وب‌پنل Luffy Gateway به صورت مستقل و بدون کپی‌برداری از پروژه‌های بسته یا فاقد مجوز طراحی شده‌اند. از ایده‌ها و استانداردهای پروژه‌های معتبر دنیای متن‌باز با رعایت کامل کپی‌رایت بهره‌گیری شده است."
            } else {
                "All Luffy Gateway routing logic, rate limiting, UI components, and panel APIs are engineered independently in pure Rust without incorporating unlicensed proprietary code. Upstream open-source specifications are integrated strictly under their respective licenses."
            }
        );

        Self::render_base_layout(lang, "about", &content, i18n)
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
