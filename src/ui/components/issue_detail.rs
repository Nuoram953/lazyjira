use super::{utils, Component};
use crate::models::{AppData, JiraIssue};
use crate::navigation::{FocusedPane, NavigationState};
use chrono::{DateTime, Utc};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct IssueDetail<'a> {
    issue: Option<&'a JiraIssue>,
    is_focused: bool,
}

impl<'a> IssueDetail<'a> {
    pub fn new(app_data: &'a AppData, nav_state: &NavigationState) -> Self {
        Self {
            issue: app_data.selected_issue.as_ref(),
            is_focused: nav_state.focused_pane == FocusedPane::Detail,
        }
    }

    pub fn with_issue(issue: Option<&'a JiraIssue>, is_focused: bool) -> Self {
        Self { issue, is_focused }
    }

    fn format_date(date: &DateTime<Utc>) -> String {
        date.format("%Y-%m-%d %H:%M UTC").to_string()
    }

    fn create_content_lines<'b>(&self, issue: &'b JiraIssue) -> Vec<Line<'b>> {
        vec![
            Line::from(vec![Span::styled(
                &issue.key,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Summary:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(issue.summary.clone()),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Status:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                &issue.status,
                utils::status_color(&issue.status),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Type:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(issue.issue_type.clone()),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Priority:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(issue.priority.clone().unwrap_or_else(|| "None".to_string())),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Assignee:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(
                issue
                    .assignee
                    .clone()
                    .unwrap_or_else(|| "Unassigned".to_string()),
            ),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Reporter:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(
                issue
                    .reporter
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string()),
            ),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Created:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(Self::format_date(&issue.created)),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Updated:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(Self::format_date(&issue.updated)),
        ]
    }

    fn add_description_lines<'b>(&self, lines: &mut Vec<Line<'b>>, issue: &'b JiraIssue) {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "Description:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]));

        if let Some(description) = &issue.description {
            if !description.trim().is_empty() {
                // Split description into multiple lines if it's long
                let desc_lines: Vec<&str> = description.lines().collect();
                for line in desc_lines {
                    lines.push(Line::from(line.to_string()));
                }
            } else {
                lines.push(Line::from("No description provided"));
            }
        } else {
            lines.push(Line::from("No description provided"));
        }
    }
}

impl<'a> Component for IssueDetail<'a> {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let border_style = utils::focus_style(self.is_focused);

        let paragraph = if let Some(issue) = self.issue {
            let mut content = self.create_content_lines(issue);
            self.add_description_lines(&mut content, issue);

            Paragraph::new(content).wrap(Wrap { trim: true }).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Issue Detail")
                    .border_style(border_style),
            )
        } else {
            Paragraph::new("No issue selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Issue Detail")
                    .border_style(border_style),
            )
        };

        frame.render_widget(paragraph, area);
    }
}
