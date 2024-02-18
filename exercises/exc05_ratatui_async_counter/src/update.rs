use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

impl App {
    pub fn update(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.quit()
                }
            }
            KeyCode::Right | KeyCode::Char('j') => self.increment_counter(),
            KeyCode::Left | KeyCode::Char('k') => self.decrement_counter(),
            _ => {}
        };
    }
}
