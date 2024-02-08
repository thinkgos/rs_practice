use anyhow::Result;

use crate::{
    event::{Event, EventHandler},
    tui::Tui,
    update::update,
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
    pub fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        let events = EventHandler::new(250);
        while !self.should_quit {
            // Render the user interface.
            tui.draw(|frame| self.render(frame))?;
            // Handle events.
            match events.next()? {
                Event::Tick => {}
                Event::Key(key_event) => update(self, key_event),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            };
        }
        Ok(())
    }
}
