use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    tui::Tui,
};

impl App {
    pub fn run(&mut self) -> Result<bool> {
        let mut tui = Tui::new()?;
        // let events = EventHandler::new(250);
        // Start the main loop.
        loop {
            // Render the user interface.
            tui.draw(|frame| self.render(frame))?;
            if let Some(t) = self.update_event()? {
                return Ok(t);
            }
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        f.render_widget(
            Paragraph::new(format!(
                "
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
          ",
                111
            ))
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            f.size(),
        )
    }

    pub fn update_event(&mut self) -> Result<Option<bool>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(None);
            }
            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        self.current_screen = CurrentScreen::Editing;
                        self.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(Some(true));
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(Some(false));
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    self.save_key_value();
                                    self.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    self.value_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.current_screen = CurrentScreen::Main;
                        self.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        self.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    self.value_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(None)
    }
}
