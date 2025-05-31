use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use schemars::JsonSchema;
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

#[derive(Serialize, Deserialize, JsonSchema)]
struct CreateEventArgs {
    summary: String,
    description: Option<String>,
    start_datetime: String,
    end_datetime: String,
    timezone: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct AddEventArgs {
    title: String,
    start_time: String,
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
        #[rmcp::tool(aggr)] args: CreateEventArgs,
    ) -> Result<CallToolResult, rmcp::Error> {
        let api_key = env::var("GOOGLE_CALENDAR_API_KEY").unwrap();
        
        let calendar_id = env::var("GOOGLE_CALENDAR_ID")
            .unwrap_or_else(|_| "primary".to_string());
        
        let tz = args.timezone.unwrap_or_else(|| "Asia/Tokyo".to_string());
        
        let event = GoogleCalendarEvent {
            summary: args.summary,
            description: args.description,
            start: EventDateTime {
                date_time: args.start_datetime,
                time_zone: tz.clone(),
            },
            end: EventDateTime {
                date_time: args.end_datetime,
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
            .await.unwrap();

        let created_event: GoogleCalendarResponse = response.json().await.unwrap();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "予定 '{}' を作成しました。ID: {}",
            created_event.summary.unwrap_or_else(|| "Unknown".to_string()),
            created_event.id.unwrap_or_else(|| "Unknown".to_string())
        ))]))
    }

    #[rmcp::tool(description = "新しい予定を追加する（簡易版）")]
    pub async fn add_event(&self, #[rmcp::tool(aggr)]args: AddEventArgs) -> Result<CallToolResult, rmcp::Error> {
        // 1時間後を終了時間とする簡易実装
        let start_dt = chrono::DateTime::parse_from_rfc3339(&args.start_time).unwrap();
        
        let end_dt = start_dt + chrono::Duration::hours(1);
        
        let result = self.create_event(CreateEventArgs {
            summary: args.title,
            description: None,
            start_datetime: start_dt.to_rfc3339(),
            end_datetime: end_dt.to_rfc3339(),
            timezone: None,
        }).await.unwrap();

        Ok(result)
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
