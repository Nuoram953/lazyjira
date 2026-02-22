use super::{utils, Component};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Footer {
    text: String,
}

impl Footer {
    pub fn new() -> Self {
        Self {
            text: "q: Quit | ?: Help | Ctrl+T: Transition | hjkl: Navigate | Enter: Select & Focus Detail | Esc: Back".to_string(),
        }
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Default for Footer {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Footer {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new(self.text.as_str())
            .style(utils::status_color("").fg(Color::Gray))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(footer, area);
    }
}
