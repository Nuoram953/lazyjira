use super::Component;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct HelpView {
    title: String,
}

impl HelpView {
    pub fn new() -> Self {
        Self {
            title: "Help".to_string(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    fn create_help_content() -> Vec<Line<'static>> {
        vec![
            Line::from(vec![Span::styled(
                "LazyJira Help",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Vim-style Navigation:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from("  h, j, k, l    - Move focus/selection (vim directions)"),
            Line::from("  H, L          - Move between left and right columns"),
            Line::from("  Enter         - Select item and focus detail view"),
            Line::from("  Esc           - Go back from detail to previous pane"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "General:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from("  q             - Quit application"),
            Line::from("  ?             - Show/hide this help"),
            Line::from("  Ctrl+T        - Show transitions for selected issue"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Panes:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from("  Sprint        - Current sprint issues"),
            Line::from("  Last Viewed   - Recently viewed issues"),
            Line::from("  Last Updated  - Recently updated issues"),
            Line::from("  Detail        - Selected issue details"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Tips:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from("• Use j/k to move up/down within a pane"),
            Line::from("• Use h/l to move between panes"),
            Line::from("• Yellow border indicates focused pane"),
            Line::from("• Press Enter to select an issue and view details"),
            Line::from("• Press Esc to return from detail view to issue list"),
        ]
    }
}

impl Default for HelpView {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for HelpView {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let content = Self::create_help_content();

        let paragraph = Paragraph::new(content).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.as_str())
                .border_style(Style::default().fg(Color::Yellow)),
        );

        frame.render_widget(paragraph, area);
    }
}
