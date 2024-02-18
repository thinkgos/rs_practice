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

use std::{
    io::{self, Stderr},
    panic,
};

use anyhow::Result;
use app::App;
use tui::Tui;

use crate::tui_event::Event;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();
    init_panic_hook();
    app.run().await?;
    Ok(())
}

/// Initializes the terminal interface panic hook.
/// the terminal properties if unexpected errors occur
fn init_panic_hook() {
    // Define a custom panic hook to reset the terminal properties.
    // This way, you won't have your terminal messed up if an unexpected error happens.
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        let tui: Result<Tui<io::Stderr, Event>> = Tui::new(io::stderr());
        if let Ok(t) = tui {
            drop(t);
        }
        panic_hook(panic);
    }));
}
