use super::{utils, Component};
use crate::models::JiraIssue;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct IssueList<'a> {
    issues: &'a [JiraIssue],
    title: String,
    selected_index: usize,
    is_focused: bool,
}

impl<'a> IssueList<'a> {
    pub fn new(
        issues: &'a [JiraIssue],
        title: impl Into<String>,
        selected_index: usize,
        is_focused: bool,
    ) -> Self {
        Self {
            issues,
            title: title.into(),
            selected_index,
            is_focused,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn focused(mut self) -> Self {
        self.is_focused = true;
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }
}

impl<'a> Component for IssueList<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let border_style = utils::focus_style(self.is_focused);

        let items: Vec<ListItem> = if self.issues.is_empty() {
            vec![ListItem::new(Span::raw("No issues"))]
        } else {
            self.issues
                .iter()
                .enumerate()
                .map(|(i, issue)| {
                    let style = if i == self.selected_index && self.is_focused {
                        Style::default()
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        utils::status_color(&issue.status)
                    };

                    let content = format!("{} - {}", issue.key, issue.summary);
                    ListItem::new(Span::styled(content, style))
                })
                .collect()
        };

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.clone())
            .border_style(border_style);

        if self.is_focused {
            block = block.title_bottom(
                Line::from(format!("{}/{}", self.selected_index + 1, self.issues.len()))
                    .right_aligned(),
            );
        }

        let list = List::new(items).block(block);

        frame.render_widget(list, area);
    }
}
