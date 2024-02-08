use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Default)]
pub enum CurrentlyEditing {
    #[default]
    Key,
    Value,
}

#[derive(Debug, Default)]
pub struct App {
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}
impl App {
    pub fn new() -> App {
        let mut p = HashMap::new();
        p.insert("k1".to_string(), "v1".to_string());
        p.insert("k2".to_string(), "v2".to_string());

        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: p,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        self.currently_editing = match &self.currently_editing {
            Some(CurrentlyEditing::Key) => Some(CurrentlyEditing::Value),
            _ => Some(CurrentlyEditing::Key),
        };
    }
    pub fn json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.pairs)?)
    }
}
