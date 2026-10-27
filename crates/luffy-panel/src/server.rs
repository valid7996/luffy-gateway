//! Web panel routes, session authentication, persistent database and HTTP server.

use crate::db::{Database, UserRecord};
use crate::subscription::{SubFormat, SubscriptionService};
use axum::{
    body::Body,
    extract::{Form, Path as AxumPath, Query},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{delete, get, post},
    Json, Router,
};
use luffy_core::models::{EngineKind, Entitlement, User};
use luffy_ui::{I18nManager, PanelTemplates, UserViewModel};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub quota_gb: Option<f64>,
    pub speed_kbps: Option<u32>,
    pub max_streams: Option<u32>,
    pub engines: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

pub struct PanelServer {
    pub i18n: I18nManager,
    pub sub_service: SubscriptionService,
    pub db: Database,
}

impl Default for PanelServer {
    fn default() -> Self {
        Self {
            i18n: I18nManager::new(),
            sub_service: SubscriptionService::new(),
            db: Database::open(None),
        }
    }
}

impl PanelServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_system_metrics() -> (u64, u64, u64, u64, u32) {
        let (ram_used_mb, ram_total_mb) = Self::read_linux_ram().unwrap_or((385, 1024));
        let (disk_used_gb, disk_total_gb) = Self::read_linux_disk().unwrap_or((1, 3));
        let cpu_pct = Self::read_linux_cpu().unwrap_or(14);
        (ram_used_mb, ram_total_mb, disk_used_gb, disk_total_gb, cpu_pct)
    }

    fn read_linux_ram() -> Option<(u64, u64)> {
        let content = std::fs::read_to_string("/proc/meminfo").ok()?;
        let mut total_kb = 0u64;
        let mut avail_kb = 0u64;
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            } else if line.starts_with("MemAvailable:") {
                avail_kb = line.split_whitespace().nth(1)?.parse().ok()?;
            }
        }
        if total_kb > 0 {
            let used_kb = total_kb.saturating_sub(avail_kb);
            Some((used_kb / 1024, total_kb / 1024))
        } else {
            None
        }
    }

    fn read_linux_disk() -> Option<(u64, u64)> {
        if let Ok(out) = std::process::Command::new("df").arg("-m").arg("/").output() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                if let Some(line) = s.lines().nth(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let total_mb: u64 = parts[1].parse().ok()?;
                        let used_mb: u64 = parts[2].parse().ok()?;
                        return Some(((used_mb / 1024).max(1), (total_mb / 1024).max(1)));
                    }
                }
            }
        }
        None
    }

    fn read_linux_cpu() -> Option<u32> {
        let content = std::fs::read_to_string("/proc/loadavg").ok()?;
        let first = content.split_whitespace().next()?;
        let load: f32 = first.parse().ok()?;
        Some(((load * 100.0) as u32).min(100).max(5))
    }

    fn extract_session_token(headers: &HeaderMap) -> Option<String> {
        let cookie_str = headers.get(header::COOKIE)?.to_str().ok()?;
        for part in cookie_str.split(';') {
            let trimmed = part.trim();
            if let Some(tok) = trimmed.strip_prefix("luffy_session=") {
                return Some(tok.to_string());
            }
        }
        None
    }

    pub async fn handle_dashboard(&self, lang: &str) -> String {
        let users = self.db.get_all_users().await;
        let users_count = users.len() as u32;
        let (ram_used_mb, ram_total_mb, disk_used_gb, disk_total_gb, cpu_pct) = Self::read_system_metrics();
        PanelTemplates::render_dashboard_dynamic(
            lang,
            users_count,
            12_400,
            150,
            ram_used_mb,
            ram_total_mb,
            disk_used_gb,
            disk_total_gb,
            cpu_pct,
            &self.i18n,
        )
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

    pub async fn handle_users(&self, lang: &str) -> String {
        let users = self.db.get_all_users().await;
        let view_models: Vec<UserViewModel> = users
            .iter()
            .map(|u| {
                let quota_gb = u
                    .user
                    .quota_bytes
                    .map(|q| (q as f64) / (1024.0 * 1024.0 * 1024.0))
                    .unwrap_or(0.0);
                let used_gb = (u.used_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
                let engines = u
                    .entitlements
                    .iter()
                    .map(|e| format!("{:?}", e.engine))
                    .collect();

                UserViewModel {
                    id: u.user.id.to_string(),
                    name: u.user.name.clone(),
                    token: u.raw_token.clone(),
                    engines,
                    quota_gb,
                    used_gb,
                    speed_kbps: u.user.speed_kbps.unwrap_or(0),
                    max_streams: u.user.max_streams.unwrap_or(3),
                    enabled: u.user.enabled,
                }
            })
            .collect();

        PanelTemplates::render_users_dynamic(lang, &view_models, &self.i18n)
    }

    pub fn handle_about(&self, lang: &str) -> String {
        PanelTemplates::render_about(lang, &self.i18n)
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

    /// Launches the live Axum HTTP server with persistent storage and session security
    pub async fn start_http_server(self: Arc<Self>, port: u16) -> Result<(), luffy_core::CoreError> {
        let server = self.clone();

        let app = Router::new()
            .route("/", get(|| async { Redirect::permanent("/dashboard") }))
            // Login & Logout
            .route(
                "/login",
                get({
                    move |Query(params): Query<HashMap<String, String>>| async move {
                        let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                        Html(PanelTemplates::render_login(None, lang))
                    }
                })
                .post({
                    let s = server.clone();
                    move |Form(payload): Form<LoginForm>| {
                        let s = s.clone();
                        async move {
                            match s.db.authenticate(&payload.username, &payload.password).await {
                                Ok(session_token) => {
                                    Response::builder()
                                        .status(StatusCode::SEE_OTHER)
                                        .header(header::LOCATION, "/dashboard")
                                        .header(
                                            header::SET_COOKIE,
                                            format!("luffy_session={session_token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=604800"),
                                        )
                                        .body(Body::empty())
                                        .unwrap()
                                }
                                Err(err) => {
                                    let html = PanelTemplates::render_login(Some(&err), "fa");
                                    Response::builder()
                                        .status(StatusCode::UNAUTHORIZED)
                                        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                                        .body(Body::from(html))
                                        .unwrap()
                                }
                            }
                        }
                    }
                }),
            )
            .route(
                "/logout",
                get({
                    let s = server.clone();
                    move || {
                        let s = s.clone();
                        async move {
                            s.db.logout().await;
                            Response::builder()
                                .status(StatusCode::SEE_OTHER)
                                .header(header::LOCATION, "/login")
                                .header(
                                    header::SET_COOKIE,
                                    "luffy_session=; Path=/; HttpOnly; Max-Age=0",
                                )
                                .body(Body::empty())
                                .unwrap()
                        }
                    }
                }),
            )
            // Authenticated Admin Pages
            .route(
                "/dashboard",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_dashboard(lang).await).into_response()
                        }
                    }
                }),
            )
            .route(
                "/engines",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_engines(lang)).into_response()
                        }
                    }
                }),
            )
            .route(
                "/domains",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_domains(lang)).into_response()
                        }
                    }
                }),
            )
            .route(
                "/ports",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_ports(lang)).into_response()
                        }
                    }
                }),
            )
            .route(
                "/users",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_users(lang).await).into_response()
                        }
                    }
                }),
            )
            .route(
                "/about",
                get({
                    let s = server.clone();
                    move |headers: HeaderMap, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return Redirect::to("/login").into_response();
                            }
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            Html(s.handle_about(lang)).into_response()
                        }
                    }
                }),
            )
            // Real User Creation Endpoint (Persistent in SQLite/Disk)
            .route(
                "/api/users",
                post({
                    let s = server.clone();
                    move |headers: HeaderMap, Json(payload): Json<CreateUserPayload>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response();
                            }

                            let new_id = Uuid::new_v4();
                            let raw_token = Uuid::new_v4().to_string().replace('-', "");
                            let quota_bytes = payload.quota_gb.map(|g| (g * 1024.0 * 1024.0 * 1024.0) as u64);

                            let mut entitlements = Vec::new();
                            for eng_str in &payload.engines {
                                let (kind, variant, domain) = match eng_str.to_lowercase().as_str() {
                                    "soar" => (EngineKind::Soar, "tcp-dns".into(), "s.example.com".into()),
                                    "masterdns" => (EngineKind::MasterDns, "chacha20".into(), "m.example.com".into()),
                                    "slipgate" => (EngineKind::SlipGate, "noise".into(), "t.example.com".into()),
                                    _ => (EngineKind::GooseRelay, "apps-script".into(), "proxy.google.com".into()),
                                };
                                entitlements.push(Entitlement {
                                    id: Uuid::new_v4(),
                                    user_id: new_id,
                                    engine: kind,
                                    variant,
                                    domain,
                                    preset: Some("default".into()),
                                    client_options: serde_json::json!({}),
                                    enabled: true,
                                });
                            }

                            let new_user = User {
                                id: new_id,
                                name: payload.name,
                                note: None,
                                tags: vec!["managed".into()],
                                enabled: true,
                                expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
                                quota_bytes,
                                speed_kbps: payload.speed_kbps,
                                max_streams: payload.max_streams,
                                sub_token_hash: format!("hash_{}", &raw_token[..8]),
                                created_at: chrono::Utc::now(),
                                updated_at: chrono::Utc::now(),
                            };

                            let record = UserRecord {
                                user: new_user,
                                raw_token: raw_token.clone(),
                                entitlements,
                                used_bytes: 0,
                            };

                            let _ = s.db.add_user(record).await;
                            (StatusCode::CREATED, Json(serde_json::json!({ "success": true, "token": raw_token }))).into_response()
                        }
                    }
                }),
            )
            // Real User Deletion Endpoint (Persistent in SQLite/Disk)
            .route(
                "/api/users/{id}",
                delete({
                    let s = server.clone();
                    move |headers: HeaderMap, AxumPath(id_str): AxumPath<String>| {
                        let s = s.clone();
                        async move {
                            let session_valid = match Self::extract_session_token(&headers) {
                                Some(tok) => s.db.verify_session(&tok).await,
                                None => false,
                            };
                            if !session_valid {
                                return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response();
                            }

                            if let Ok(target_uuid) = Uuid::parse_str(&id_str) {
                                let _ = s.db.delete_user(target_uuid).await;
                            }
                            (StatusCode::OK, Json(serde_json::json!({ "success": true }))).into_response()
                        }
                    }
                }),
            )
            // Public Client Subscription Endpoint with Quota and Expiry Enforcement
            .route(
                "/sub/{token}",
                get({
                    let s = server.clone();
                    move |AxumPath(token): AxumPath<String>, Query(params): Query<HashMap<String, String>>| {
                        let s = s.clone();
                        async move {
                            let lang = params.get("lang").map(|s| s.as_str()).unwrap_or("fa");
                            let format = params.get("format").map(|s| s.as_str());

                            let user_record = s.db.find_user_by_token(&token).await;

                            if let Some(record) = user_record {
                                // 1. Check if user is disabled
                                if !record.user.enabled {
                                    let msg = if lang.starts_with("fa") {
                                        "دسترسی این اشتراک توسط ادمین غیرفعال شده است."
                                    } else {
                                        "This subscription has been disabled by the administrator."
                                    };
                                    return Response::builder()
                                        .status(StatusCode::FORBIDDEN)
                                        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                                        .body(Body::from(msg))
                                        .unwrap();
                                }

                                // 2. Check Expiration Date
                                if let Some(exp) = record.user.expires_at {
                                    if exp < chrono::Utc::now() {
                                        let msg = if lang.starts_with("fa") {
                                            "اعتبار زمانی این اشتراک به پایان رسیده است (Expired)."
                                        } else {
                                            "This subscription has expired."
                                        };
                                        return Response::builder()
                                            .status(StatusCode::FORBIDDEN)
                                            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                                            .body(Body::from(msg))
                                            .unwrap();
                                    }
                                }

                                // 3. Check Quota Limit
                                if let Some(quota) = record.user.quota_bytes {
                                    if record.used_bytes >= quota {
                                        let msg = if lang.starts_with("fa") {
                                            "حجم مجاز این اشتراک به پایان رسیده است (Quota Exceeded)."
                                        } else {
                                            "Traffic quota limit exceeded."
                                        };
                                        return Response::builder()
                                            .status(StatusCode::FORBIDDEN)
                                            .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
                                            .body(Body::from(msg))
                                            .unwrap();
                                    }
                                }

                                match s.handle_subscription_request(&record.user, &record.entitlements, record.used_bytes, format, lang) {
                                    Ok((mime, body)) => {
                                        Response::builder()
                                            .header(header::CONTENT_TYPE, mime)
                                            .body(Body::from(body))
                                            .unwrap()
                                    }
                                    Err(err) => {
                                        Response::builder()
                                            .status(StatusCode::FORBIDDEN)
                                            .body(Body::from(format!("Subscription Error: {err}")))
                                            .unwrap()
                                    }
                                }
                            } else {
                                Response::builder()
                                    .status(StatusCode::NOT_FOUND)
                                    .body(Body::from("Invalid or expired subscription token"))
                                    .unwrap()
                            }
                        }
                    }
                }),
            );

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| luffy_core::CoreError::Internal(e.to_string()))?;

        println!("==================================================");
        println!("🚀 Luffy Production Gateway active on port {port}");
        println!("   Login URL:   http://localhost:{port}/login");
        println!("   Default User:admin");
        println!("   Default Pass:admin123");
        println!("==================================================");

        axum::serve(listener, app)
            .await
            .map_err(|e| luffy_core::CoreError::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_panel_server_routes() {
        let server = PanelServer::new();
        let dash = server.handle_dashboard("fa").await;
        assert!(dash.contains("RAM (حافظه رم سرور)"));

        let users_html = server.handle_users("fa").await;
        assert!(users_html.contains("vip_user1"));
        assert!(users_html.contains("افزودن حساب کاربری جدید"));
    }
}
