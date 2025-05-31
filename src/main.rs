use rmcp::ServiceExt;

pub mod google_calendar;
use google_calendar::GoogleCalendar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Google Calendar MCP...");

    let service = GoogleCalendar::new().serve(rmcp::transport::stdio()).await?;

    service.waiting().await?;

    Ok(())
}
