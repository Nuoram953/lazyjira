pub mod footer;
pub mod help_view;
pub mod issue_detail;
pub mod issue_list;
pub mod modal;
pub mod transition_selector;

use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub mod utils {
    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::{Color, Style},
    };

    pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    pub fn status_color(status: &str) -> Style {
        match status {
            "To Do" => Style::default().fg(Color::Gray),
            "In Progress" => Style::default().fg(Color::Yellow),
            "Done" => Style::default().fg(Color::Green),
            "Blocked" => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::White),
        }
    }

    pub fn focus_style(is_focused: bool) -> Style {
        if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        }
    }
}
