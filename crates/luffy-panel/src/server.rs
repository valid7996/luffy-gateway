//! Web panel routes and HTTP handlers for Luffy Gateway.

use crate::subscription::{SubFormat, SubscriptionService};
use luffy_core::models::{Entitlement, User};
use luffy_ui::{I18nManager, PanelTemplates};

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
