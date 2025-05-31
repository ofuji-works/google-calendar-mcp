use rmcp::model::{ServerCapabilities, ServerInfo};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(derive_new::new, Clone)]
pub struct GoogleCalendar;

#[derive(Serialize, Deserialize)]
struct GoogleCalendarEvent {
    summary: String,
    description: Option<String>,
    start: EventDateTime,
    end: EventDateTime,
}

#[derive(Serialize, Deserialize)]
struct EventDateTime {
    #[serde(rename = "dateTime")]
    date_time: String,
    #[serde(rename = "timeZone")]
    time_zone: String,
}

#[derive(Serialize, Deserialize)]
struct GoogleCalendarResponse {
    id: Option<String>,
    summary: Option<String>,
    #[serde(rename = "htmlLink")]
    html_link: Option<String>,
}

#[rmcp::tool(tool_box)]
impl GoogleCalendar {
    #[rmcp::tool(description = "Googleカレンダーの予定一覧を取得する")]
    pub fn list_events(&self) -> String {
        // TODO: GoogleカレンダーAPI連携実装
        "予定一覧のダミーデータ".to_string()
    }

    #[rmcp::tool(description = "新しい予定を作成する")]
    pub async fn create_event(
        &self,
        summary: String,
        description: Option<String>,
        start_datetime: String,
        end_datetime: String,
        timezone: Option<String>,
    ) -> anyhow::Result<String> {
        let api_key = env::var("GOOGLE_CALENDAR_API_KEY")
            .map_err(|_| anyhow::anyhow!("GOOGLE_CALENDAR_API_KEY environment variable not set"))?;
        
        let calendar_id = env::var("GOOGLE_CALENDAR_ID")
            .unwrap_or_else(|_| "primary".to_string());
        
        let tz = timezone.unwrap_or_else(|| "Asia/Tokyo".to_string());
        
        let event = GoogleCalendarEvent {
            summary,
            description,
            start: EventDateTime {
                date_time: start_datetime,
                time_zone: tz.clone(),
            },
            end: EventDateTime {
                date_time: end_datetime,
                time_zone: tz,
            },
        };

        let client = reqwest::Client::new();
        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/{}/events?key={}",
            calendar_id, api_key
        );

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&event)
            .send()
            .await?;

        if response.status().is_success() {
            let created_event: GoogleCalendarResponse = response.json().await?;
            Ok(format!(
                "予定 '{}' を作成しました。ID: {}",
                created_event.summary.unwrap_or_else(|| "Unknown".to_string()),
                created_event.id.unwrap_or_else(|| "Unknown".to_string())
            ))
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Google Calendar API error: {}", error_text))
        }
    }

    #[rmcp::tool(description = "新しい予定を追加する（簡易版）")]
    pub async fn add_event(&self, title: String, start_time: String) -> anyhow::Result<String> {
        // 1時間後を終了時間とする簡易実装
        let start_dt = chrono::DateTime::parse_from_rfc3339(&start_time)
            .map_err(|_| anyhow::anyhow!("Invalid start_time format. Use RFC3339 format (e.g., 2023-12-25T10:00:00+09:00)"))?;
        
        let end_dt = start_dt + chrono::Duration::hours(1);
        
        self.create_event(
            title,
            None,
            start_dt.to_rfc3339(),
            end_dt.to_rfc3339(),
            None,
        ).await
    }
}

#[rmcp::tool(tool_box)]
impl rmcp::ServerHandler for GoogleCalendar {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Google Calendar MCP".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
