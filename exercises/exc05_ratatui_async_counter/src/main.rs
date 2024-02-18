/// Application.
pub mod app;
/// Application updater.do
pub mod app_update;
/// Terminal user interface.
pub mod tui;
/// Terminal events handler.
pub mod tui_event;
/// Widget renderer.
pub mod ui;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();
    app.run().await?;
    Ok(())
}
