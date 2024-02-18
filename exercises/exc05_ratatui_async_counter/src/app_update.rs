use crate::app::{Action, App};

impl App {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.quit(),
            Action::Increment => self.increment_counter(),
            Action::Decrement => self.decrement_counter(),
            Action::Tick => {}
            Action::None => {}
        };
    }
}
