//! Web panel routes and HTTP server for Luffy Gateway.

use crate::subscription::{SubFormat, SubscriptionService};
use axum::{
    body::Body,
    extract::{Path as AxumPath, Query},
    response::{Html, Redirect, Response},
    routing::get,
    Router,
};
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_ui::{I18nManager, PanelTemplates};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

pub struct PanelServer {
    pub i18n: I18nManager,
    pub sub_service: SubscriptionService,
}

impl Default for PanelServer {
    fn default() -> Self {
        Self {
            i18n: I18nManager::new(),
            sub_service: SubscriptionService::new(),
        }
    }
}

impl PanelServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_dashboard(&self, lang: &str) -> String {
        PanelTemplates::render_dashboard(lang, 42, 12_400, 150, &self.i18n)
    }

    pub fn handle_engines(&self, lang: &str) -> String {
        PanelTemplates::render_engines(lang, &self.i18n)
    }

    pub fn handle_domains(&self, lang: &str) -> String {
        PanelTemplates::render_domains(lang, &self.i18n)
    }

    pub fn handle_ports(&self, lang: &str) -> String {
        PanelTemplates::render_ports(lang, &self.i18n)
    }

    pub fn handle_users(&self, lang: &str) -> String {
        PanelTemplates::render_users(lang, &self.i18n)
    }

    pub fn handle_about(&self, lang: &str) -> String {
        let is_fa = lang.starts_with("fa");
        let content = if is_fa {
            r#"<div class="card">
                <h2>درباره و پروژه‌های بالادستی (Upstream Credits)</h2>
                <p style="margin-top: 12px; color: var(--text-secondary);">
                    سامانه Luffy Gateway با رعایت استانداردهای Clean-Room و ادای احترام به نویسندگان پروژه‌های زیر توسعه یافته است:
                </p>
                <div style="margin-top: 20px;">
                    <table>
                        <thead>
                            <tr><th>پروژه</th><th>نویسنده</th><th>لایسنس</th><th>نقش</th></tr>
                        </thead>
                        <tbody>
                            <tr><td>Lantern Soar / soar-rs</td><td>Lantern</td><td>Apache-2.0</td><td>هسته تونل Soar</td></tr>
                            <tr><td>MasterDnsVPN</td><td>masterking32</td><td>MIT</td><td>هسته تونل MasterDNS</td></tr>
                            <tr><td>SlipGate & SlipNet</td><td>anonvector</td><td>AGPL-3.0</td><td>سرور و پروفایل‌های SlipNet</td></tr>
                            <tr><td>GooseRelay / MooseRelay</td><td>Kianmhz / easyfast2008</td><td>MIT</td><td>رله Google Apps Script</td></tr>
                            <tr><td>Vazirmatn Font</td><td>Saber Rastikerdar</td><td>SIL OFL 1.1</td><td>فونت وب رابط کاربری</td></tr>
                        </tbody>
                    </table>
                </div>
            </div>"#
        } else {
            r#"<div class="card">
                <h2>About & Upstream Credits</h2>
                <p style="margin-top: 12px; color: var(--text-secondary);">
                    Luffy Gateway is built clean-room with deep gratitude and attribution to the following upstream authors:
                </p>
                <div style="margin-top: 20px;">
                    <table>
                        <thead>
                            <tr><th>Project</th><th>Author</th><th>License</th><th>Role</th></tr>
                        </thead>
                        <tbody>
                            <tr><td>Lantern Soar / soar-rs</td><td>Lantern</td><td>Apache-2.0</td><td>Soar Tunnel Engine</td></tr>
                            <tr><td>MasterDnsVPN</td><td>masterking32</td><td>MIT</td><td>MasterDNS Tunnel Engine</td></tr>
                            <tr><td>SlipGate & SlipNet</td><td>anonvector</td><td>AGPL-3.0</td><td>SlipNet Server & Profiles</td></tr>
                            <tr><td>GooseRelay / MooseRelay</td><td>Kianmhz / easyfast2008</td><td>MIT</td><td>Google Apps Script Relay</td></tr>
                            <tr><td>Vazirmatn Font</td><td>Saber Rastikerdar</td><td>SIL OFL 1.1</td><td>UI Web Typography</td></tr>
                        </tbody>
                    </table>
                </div>
            </div>"#
        };

        PanelTemplates::render_base_layout(lang, "about", content, &self.i18n)
    }

    pub fn handle_subscription_request(
        &self,
        user: &User,
        entitlements: &[Entitlement],
        used_bytes: u64,
        format_param: Option<&str>,
        lang: &str,
    ) -> Result<(String, String), luffy_core::CoreError> {
        let fmt = SubFormat::from_query(format_param);
        self.sub_service
            .render_subscription(user, entitlements, used_bytes, fmt, lang)
    }

    /// Launches the live Axum HTTP server on the given port
    pub async fn start_http_server(self: Arc<Self>, port: u16) -> Result<(), luffy_core::CoreError> {
        let server = self.clone();

        let app = Router::new()
            .route("/", get(|| async { Redirect::permanent("/dashboard") }))
            .route(
                "/dashboard",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_dashboard(lang))
                        }
                    }
                }),
            )
            .route(
                "/engines",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_engines(lang))
                        }
                    }
                }),
            )
            .route(
                "/domains",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_domains(lang))
                        }
                    }
                }),
            )
            .route(
                "/ports",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_ports(lang))
                        }
                    }
                }),
            )
            .route(
                "/users",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_users(lang))
                        }
                    }
                }),
            )
            .route(
                "/about",
                get({
                    let s = server.clone();
                    move |Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_about(lang))
                        }
                    }
                }),
            )
            .route(
                "/sub/{token}",
                get({
                    let s = server.clone();
                    move |AxumPath(_token): AxumPath<String>, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            let format = params.get("format").map(|s| s.as_str());

                            let demo_user = User {
                                id: Uuid::new_v4(),
                                name: "Demo User".to_string(),
                                note: None,
                                tags: vec!["vip".into()],
                                enabled: true,
                                expires_at: None,
                                quota_bytes: Some(20 * 1024 * 1024 * 1024),
                                speed_kbps: Some(4000),
                                max_streams: Some(5),
                                sub_token_hash: "hash".into(),
                                created_at: chrono::Utc::now(),
                                updated_at: chrono::Utc::now(),
                            };

                            let entitlements = vec![
                                Entitlement {
                                    id: Uuid::new_v4(),
                                    user_id: demo_user.id,
                                    engine: EngineKind::Soar,
                                    variant: "tcp-dns".into(),
                                    domain: "s.example.com".into(),
                                    preset: Some("stability".into()),
                                    client_options: serde_json::json!({}),
                                    enabled: true,
                                },
                                Entitlement {
                                    id: Uuid::new_v4(),
                                    user_id: demo_user.id,
                                    engine: EngineKind::SlipGate,
                                    variant: "dnstt".into(),
                                    domain: "t.example.com".into(),
                                    preset: None,
                                    client_options: serde_json::json!({ "query_size_preset": "80" }),
                                    enabled: true,
                                },
                            ];

                                match s.handle_subscription_request(&demo_user, &entitlements, 4 * 1024 * 1024 * 1024, format, lang) {
                                    Ok((mime, body)) => {
                                        Response::builder()
                                            .header("Content-Type", mime)
                                            .body(Body::from(body))
                                            .unwrap()
                                    }
                                    Err(err) => {
                                        Response::builder()
                                            .status(403)
                                            .body(Body::from(format!("Subscription Error: {err}")))
                                            .unwrap()
                                    }
                                }
                        }
                    }
                }),
            );

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| luffy_core::CoreError::Internal(e.to_string()))?;

        println!("--------------------------------------------------");
        println!("🚀 Luffy Gateway Live Web Panel active on:");
        println!("   http://localhost:{port}/dashboard");
        println!("   http://127.0.0.1:{port}/dashboard");
        println!("--------------------------------------------------");

        axum::serve(listener, app)
            .await
            .map_err(|e| luffy_core::CoreError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_server_routes() {
        let server = PanelServer::new();

        let dash_fa = server.handle_dashboard("fa");
        assert!(dash_fa.contains("داشبورد"));

        let about_en = server.handle_about("en");
        assert!(about_en.contains("Upstream Credits"));
        assert!(about_en.contains("SlipGate"));
    }
}
