use super::{utils, Component};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};

pub struct Modal<T: Component> {
    content: T,
    title: String,
    width_percent: u16,
    height_percent: u16,
}

impl<T: Component> Modal<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            title: "Modal".to_string(),
            width_percent: 60,
            height_percent: 70,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size(mut self, width_percent: u16, height_percent: u16) -> Self {
        self.width_percent = width_percent;
        self.height_percent = height_percent;
        self
    }
}

impl<T: Component> Component for Modal<T> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let popup_area = utils::centered_rect(self.width_percent, self.height_percent, area);

        let overlay_block = Block::default().style(Style::default().bg(Color::Reset));
        frame.render_widget(overlay_block, area);

        frame.render_widget(Clear, popup_area);

        let modal_block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Black));
        frame.render_widget(modal_block, popup_area);

        let content_area = Rect {
            x: popup_area.x + 1,
            y: popup_area.y + 1,
            width: popup_area.width.saturating_sub(2),
            height: popup_area.height.saturating_sub(2),
        };

        self.content.render(frame, content_area);
    }
}
