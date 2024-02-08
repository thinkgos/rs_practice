use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

impl App {
    pub fn render(&mut self, f: &mut Frame) {
        f.render_widget(
            Paragraph::new(format!(
                "
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
          ",
                self.counter
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
}
