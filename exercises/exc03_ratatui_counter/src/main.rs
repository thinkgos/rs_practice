/// Application.
pub mod app;
/// Terminal events handler.
pub mod event;
/// Terminal user interface.
pub mod tui;
/// Widget renderer.
pub mod ui;
/// Application updater.
pub mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use tui::Tui;

fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();
    let mut tui = Tui::new()?;
    let events = EventHandler::new(250);
    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(|frame| app.render(frame))?;
        // Handle events.
        match events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => app.update(key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    Ok(())
}
