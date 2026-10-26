//! Google Apps Script Code.gs generator and quota estimator for GooseRelay.

pub struct AppsScriptHelper;

impl AppsScriptHelper {
    /// Renders Code.gs JavaScript for deployment on Google Apps Script
    pub fn render_code_gs(vps_url: &str, tunnel_key: &str) -> String {
        format!(
            r#"// Luffy Gateway - GooseRelay Google Apps Script Relay
// Auto-generated. Do not modify VPS_URL manually.

const VPS_URL = "{vps_url}";
const TUNNEL_KEY = "{tunnel_key}";

function doPost(e) {{
  try {{
    const payload = e.postData.contents;
    const response = UrlFetchApp.fetch(VPS_URL + "/tunnel", {{
      method: "post",
      payload: payload,
      headers: {{
        "X-Tunnel-Key": TUNNEL_KEY,
        "Content-Type": "application/octet-stream"
      }},
      muteHttpExceptions: true
    }});

    return ContentService.createTextOutput(response.getContentText())
      .setMimeType(ContentService.MimeType.TEXT);
  }} catch (err) {{
    return ContentService.createTextOutput("ERROR: " + err.message)
      .setMimeType(ContentService.MimeType.TEXT);
  }}
}}

function doGet(e) {{
  return ContentService.createTextOutput("GooseRelay Apps Script Active");
}}
"#
        )
    }

    /// Calculates daily quota consumption percentage based on standard 20,000 calls/day
    pub fn estimate_quota_usage(daily_calls: u64) -> (f32, bool) {
        const DAILY_FREE_LIMIT: u64 = 20_000;
        let percentage = (daily_calls as f32 / DAILY_FREE_LIMIT as f32) * 100.0;
        let warning_threshold = percentage >= 80.0;
        (percentage, warning_threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apps_script_generation() {
        let code = AppsScriptHelper::render_code_gs("https://myvps.com:8443", "secret_key_123");
        assert!(code.contains("const VPS_URL = \"https://myvps.com:8443\";"));
        assert!(code.contains("const TUNNEL_KEY = \"secret_key_123\";"));
        assert!(code.contains("UrlFetchApp.fetch"));
    }

    #[test]
    fn test_quota_estimation() {
        let (pct, warn) = AppsScriptHelper::estimate_quota_usage(18_000);
        assert_eq!(pct, 90.0);
        assert!(warn);

        let (pct2, warn2) = AppsScriptHelper::estimate_quota_usage(5_000);
        assert_eq!(pct2, 25.0);
        assert!(!warn2);
    }
}
