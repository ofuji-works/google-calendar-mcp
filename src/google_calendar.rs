use rmcp::model::{ServerCapabilities, ServerInfo};

#[derive(derive_new::new, Clone)]
pub struct GoogleCalendar;

#[rmcp::tool(tool_box)]
impl GoogleCalendar {
    #[rmcp::tool(description = "Googleカレンダーの予定一覧を取得する")]
    pub fn list_events(&self) -> String {
        // TODO: GoogleカレンダーAPI連携実装
        "予定一覧のダミーデータ".to_string()
    }

    #[rmcp::tool(description = "新しい予定を追加する")]
    pub fn add_event(&self) -> String {
        // TODO: GoogleカレンダーAPI連携実装
        format!("予定 '{}' を追加しました", "新しい予定")
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
