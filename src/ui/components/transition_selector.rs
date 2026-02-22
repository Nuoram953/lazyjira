use super::{modal::Modal, Component};
use crate::api::types::JiraTransition;
use crate::models::AppData;
use crate::navigation::NavigationState;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

pub struct TransitionSelector<'a> {
    transitions: &'a [JiraTransition],
    selected_index: usize,
    issue_key: String,
}

impl<'a> TransitionSelector<'a> {
    pub fn new(
        transitions: &'a [JiraTransition],
        nav_state: &NavigationState,
        app_data: &AppData,
    ) -> Self {
        let issue_key = if let Some(issue) = &app_data.selected_issue {
            issue.key.clone()
        } else {
            "Unknown Issue".to_string()
        };

        Self {
            transitions,
            selected_index: nav_state.transition_selected,
            issue_key,
        }
    }

    pub fn with_transitions(
        transitions: &'a [JiraTransition],
        selected_index: usize,
        issue_key: impl Into<String>,
    ) -> Self {
        Self {
            transitions,
            selected_index,
            issue_key: issue_key.into(),
        }
    }
}

impl<'a> Component for TransitionSelector<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let content = TransitionSelectorContent {
            transitions: self.transitions,
            selected_index: self.selected_index,
        };

        let modal = Modal::new(content)
            .with_title(format!("Transition Issue: {}", self.issue_key))
            .with_size(60, 50);

        modal.render(frame, area);

        let instructions_area = Rect {
            x: area.x + area.width / 4,
            y: area.y + (area.height * 3) / 4,
            width: area.width / 2,
            height: 1,
        };

        if instructions_area.y < area.height {
            let instructions =
                Paragraph::new("/: Search | ↑↓: Navigate | Enter: Execute | Esc: Cancel")
                    .style(Style::default().fg(Color::Gray));
            frame.render_widget(instructions, instructions_area);
        }
    }
}

struct TransitionSelectorContent<'a> {
    transitions: &'a [JiraTransition],
    selected_index: usize,
}

impl<'a> Component for TransitionSelectorContent<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = if self.transitions.is_empty() {
            vec![ListItem::new(Span::raw("No transitions available"))]
        } else {
            self.transitions
                .iter()
                .enumerate()
                .map(|(i, transition)| {
                    let style = if i == self.selected_index {
                        Style::default()
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    let content = format!("{} → {}", transition.name, transition.to.name);
                    ListItem::new(Line::from(vec![Span::styled(content, style)]))
                })
                .collect()
        };

        let list = List::new(items);
        frame.render_widget(list, area);
    }
}
