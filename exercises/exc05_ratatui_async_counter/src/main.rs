/// Application.
pub mod app;
/// Terminal events handler.
pub mod event;
/// Terminal user interface.
pub mod tui;
/// Widget renderer.
pub mod ui;
/// Application updater.do
pub mod update;

use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();
    app.run().await?;
    Ok(())
}
