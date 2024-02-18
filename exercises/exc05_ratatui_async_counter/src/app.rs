use anyhow::Result;

use crate::{
    event::{poll_event, Event},
    tui::Tui,
};

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }
    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.start_poll_event(poll_event);
        while !self.should_quit {
            // Render the user interface.
            tui.draw(|frame| self.render_ui(frame))?;
            // Handle events.
            match tui.recv_event().await? {
                Event::Tick => {}
                Event::Key(key_event) => self.update(key_event),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            };
        }
        Ok(())
    }
}
