use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::services::types::{JiraIssue, JiraIssueRef};
use crate::ui::components::issue_list::icons::PriorityIcons;

#[derive(Debug, Clone)]
pub enum IssueItemMode {
    Full,

    Summary,
}

pub struct IssueItemRenderer {
    priority_icons: PriorityIcons,
}

impl IssueItemRenderer {
    pub fn new() -> Self {
        Self {
            priority_icons: PriorityIcons::new(),
        }
    }

    pub fn render_issue(
        &self,
        issue: &JiraIssue,
        mode: IssueItemMode,
        available_width: u16,
    ) -> ListItem<'_> {
        match mode {
            IssueItemMode::Full => self.render_full_issue(issue, available_width),
            IssueItemMode::Summary => self.render_summary_issue(issue, available_width),
        }
    }

    pub fn render_issue_ref(
        &self,
        issue_ref: &JiraIssueRef,
        mode: IssueItemMode,
        available_width: u16,
    ) -> ListItem<'_> {
        match mode {
            IssueItemMode::Full => self.render_full_issue_ref(issue_ref, available_width),
            IssueItemMode::Summary => self.render_summary_issue_ref(issue_ref, available_width),
        }
    }

    fn render_full_issue(&self, issue: &JiraIssue, available_width: u16) -> ListItem<'_> {
        let icon = self.priority_icons.get_icon(issue.priority.as_ref());
        let key_part = format!("[{}] ", issue.key);
        let available_width = available_width.saturating_sub(3) as usize;
        let summary_max = available_width.saturating_sub(key_part.chars().count());

        let truncated_summary = self.truncate_text(&issue.summary, summary_max);

        let mut lines = vec![Line::from(vec![
            Span::styled(key_part, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                truncated_summary,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])];

        lines.push(Line::from(vec![Span::styled(
            format!(
                "{} | {} | {} {} | {}",
                issue.issue_type,
                issue.status,
                icon,
                issue.priority.as_deref().unwrap_or("N/A"),
                issue.assignee.as_deref().unwrap_or("N/A")
            ),
            Style::default().add_modifier(Modifier::DIM),
        )]));
        lines.push(Line::from(""));

        ListItem::new(lines)
    }

    fn render_summary_issue(&self, issue: &JiraIssue, available_width: u16) -> ListItem<'_> {
        let key_part = format!("[{}] ", issue.key);
        let available_width = available_width.saturating_sub(3) as usize;
        let summary_max = available_width.saturating_sub(key_part.chars().count());

        let truncated_summary = self.truncate_text(&issue.summary, summary_max);

        let line = Line::from(vec![
            Span::styled(key_part, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                truncated_summary,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]);

        ListItem::new(vec![line])
    }

    fn render_full_issue_ref(
        &self,
        issue_ref: &JiraIssueRef,
        available_width: u16,
    ) -> ListItem<'_> {
        let key_part = format!("[{}] ", issue_ref.key);
        let available_width = available_width.saturating_sub(3) as usize;
        let summary_max = available_width.saturating_sub(key_part.chars().count());

        let summary = issue_ref.summary.as_deref().unwrap_or("No summary");
        let truncated_summary = self.truncate_text(summary, summary_max);

        let mut lines = vec![Line::from(vec![
            Span::styled(key_part, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                truncated_summary,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ])];

        let issue_type = issue_ref.issue_type.as_deref().unwrap_or("Unknown");
        lines.push(Line::from(vec![Span::styled(
            issue_type.to_string(),
            Style::default().add_modifier(Modifier::DIM),
        )]));
        lines.push(Line::from(""));

        ListItem::new(lines)
    }

    fn render_summary_issue_ref(
        &self,
        issue_ref: &JiraIssueRef,
        available_width: u16,
    ) -> ListItem<'_> {
        let key_part = format!("[{}] ", issue_ref.key);
        let available_width = available_width.saturating_sub(3) as usize;
        let summary_max = available_width.saturating_sub(key_part.chars().count());

        let summary = issue_ref.summary.as_deref().unwrap_or("No summary");
        let truncated_summary = self.truncate_text(summary, summary_max);

        let line = Line::from(vec![
            Span::styled(key_part, Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                truncated_summary,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]);

        ListItem::new(vec![line])
    }

    fn truncate_text(&self, text: &str, max_length: usize) -> String {
        if text.chars().count() > max_length {
            let mut s = text
                .chars()
                .take(max_length.saturating_sub(1))
                .collect::<String>();
            s.push('…');
            s
        } else {
            text.to_string()
        }
    }
}

impl Default for IssueItemRenderer {
    fn default() -> Self {
        Self::new()
    }
}
