use crate::{
    services::{adf, types::JiraIssue},
    ui::components::{IssueItemMode, IssueItemRenderer},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailField {
    Key,
    Summary,
    Details,
    Description,
    Parent,
    Subtasks,
    LinkedItems,
}

impl DetailField {
    pub fn next(&self) -> Self {
        match self {
            DetailField::Key => DetailField::Summary,
            DetailField::Summary => DetailField::Details,
            DetailField::Details => DetailField::Description,
            DetailField::Description => DetailField::Parent,
            DetailField::Parent => DetailField::Subtasks,
            DetailField::Subtasks => DetailField::LinkedItems,
            DetailField::LinkedItems => DetailField::Key,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            DetailField::Key => DetailField::LinkedItems,
            DetailField::Summary => DetailField::Key,
            DetailField::Details => DetailField::Summary,
            DetailField::Description => DetailField::Details,
            DetailField::Parent => DetailField::Description,
            DetailField::Subtasks => DetailField::Parent,
            DetailField::LinkedItems => DetailField::Subtasks,
        }
    }
}

pub struct IssueDetail {
    pub issue: Option<JiraIssue>,
    pub scroll_offset: usize,
    pub focused: bool,
    pub selected_field: DetailField,
    pub edit_mode: bool,
    issue_renderer: IssueItemRenderer,
}

impl Default for IssueDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl IssueDetail {
    pub fn new() -> Self {
        Self {
            issue: None,
            scroll_offset: 0,
            focused: false,
            selected_field: DetailField::Key,
            edit_mode: false,
            issue_renderer: IssueItemRenderer::new(),
        }
    }

    pub fn move_up(&mut self) {
        self.selected_field = self.selected_field.prev();
    }

    pub fn move_down(&mut self) {
        self.selected_field = self.selected_field.next();
    }

    pub fn enter_edit_mode(&mut self) {
        if !self.edit_mode {
            self.edit_mode = true;
        }
    }

    pub fn exit_edit_mode(&mut self) {
        self.edit_mode = false;
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) {
        let outer_block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::proportional(1))
            .title("Issue Detail")
            .border_style(if focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            });

        let inner_area = outer_block.inner(area);
        f.render_widget(outer_block, area);

        if let Some(_issue) = &self.issue {
            let section_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Min(0),
                    Constraint::Min(0),
                ])
                .split(inner_area);

            self.render_summary_section(f, section_chunks[0]);
            self.render_details_section(f, section_chunks[1]);
            self.render_description_section(f, section_chunks[2]);
            self.render_subtasks_section(f, section_chunks[3]);
        } else {
            f.render_widget(Paragraph::new("No issue selected"), inner_area);
        }
    }

    fn render_title_section(&self, title: &str, field: DetailField) -> Line<'_> {
        let is_selected = self.focused && self.selected_field == field;
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD)
        };
        Line::from(vec![Span::styled(format!(" {} ", title), style)])
    }

    fn render_summary_section(&self, f: &mut Frame, area: Rect) {
        let title = self.render_title_section("Summary", DetailField::Summary);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        f.render_widget(Paragraph::new(title), main_chunks[0]);

        let line_color = Color::Yellow;

        let separator = Line::from(vec![Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(line_color),
        )]);
        f.render_widget(Paragraph::new(separator), main_chunks[1]);

        if let Some(issue) = &self.issue {
            let summary = vec![
                Line::from(vec![
                    Span::styled("Key: ", Style::default().fg(Color::White)),
                    Span::raw(&issue.key),
                ]),
                Line::from(vec![
                    Span::styled("Summary: ", Style::default().fg(Color::White)),
                    Span::raw(&issue.summary),
                ]),
                Line::from(vec![
                    Span::styled("Parent: ", Style::default().fg(Color::White)),
                    Span::raw(issue.priority.as_deref().unwrap_or("None")),
                ]),
            ];

            let style = if self.focused && self.selected_field == DetailField::Summary {
                Style::default().bg(Color::Yellow)
            } else {
                Style::default()
            };

            let summary_paragraph = Paragraph::new(summary)
                .wrap(Wrap { trim: true })
                .style(style);

            f.render_widget(summary_paragraph, main_chunks[2]);
        }
    }

    fn render_details_section(&self, f: &mut Frame, area: Rect) {
        let title = self.render_title_section("Details", DetailField::Details);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        f.render_widget(Paragraph::new(title), main_chunks[0]);

        let line_color = Color::Yellow;

        let separator = Line::from(vec![Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(line_color),
        )]);
        f.render_widget(Paragraph::new(separator), main_chunks[1]);

        let detail_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[2]);

        if let Some(issue) = &self.issue {
            let left_details = vec![
                Line::from(vec![
                    Span::styled("Status: ", Style::default().fg(Color::White)),
                    Span::raw(&issue.status),
                ]),
                Line::from(vec![
                    Span::styled("Type: ", Style::default().fg(Color::White)),
                    Span::raw(&issue.issue_type),
                ]),
                Line::from(vec![
                    Span::styled("Priority: ", Style::default().fg(Color::White)),
                    Span::raw(issue.priority.as_deref().unwrap_or("None")),
                ]),
                Line::from(vec![
                    Span::styled("Parent: ", Style::default().fg(Color::White)),
                    Span::raw(issue.priority.as_deref().unwrap_or("None")),
                ]),
            ];

            let right_details = vec![
                Line::from(vec![
                    Span::styled("Reporter: ", Style::default().fg(Color::White)),
                    Span::raw(issue.reporter.as_deref().unwrap_or("Unknown")),
                ]),
                Line::from(vec![
                    Span::styled("Assignee: ", Style::default().fg(Color::White)),
                    Span::raw(issue.assignee.as_deref().unwrap_or("Unassigned")),
                ]),
                Line::from(vec![
                    Span::styled("Created: ", Style::default().fg(Color::White)),
                    Span::raw(issue.created.format("%Y-%m-%d %H:%M").to_string()),
                ]),
                Line::from(vec![
                    Span::styled("Updated: ", Style::default().fg(Color::White)),
                    Span::raw(issue.updated.format("%Y-%m-%d %H:%M").to_string()),
                ]),
            ];

            let style = if self.focused && self.selected_field == DetailField::Details {
                Style::default().bg(Color::Yellow)
            } else {
                Style::default()
            };

            let left_paragraph = Paragraph::new(left_details)
                .wrap(Wrap { trim: true })
                .style(style);

            let right_paragraph = Paragraph::new(right_details)
                .wrap(Wrap { trim: true })
                .style(style);

            f.render_widget(left_paragraph, detail_chunks[0]);
            f.render_widget(right_paragraph, detail_chunks[1]);
        }
    }

    fn render_description_section(&self, f: &mut Frame, area: Rect) {
        let title = self.render_title_section("Description", DetailField::Description);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        f.render_widget(Paragraph::new(title), main_chunks[0]);

        let line_color = Color::Yellow;

        let separator = Line::from(vec![Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(line_color),
        )]);
        f.render_widget(Paragraph::new(separator), main_chunks[1]);

        if let Some(issue) = &self.issue {
            // Parse the description using the ADF parser
            let description_text = adf::parse_description(issue.description.as_ref());

            let description_lines: Vec<Line> = description_text
                .lines()
                .map(|line| Line::from(line.to_string()))
                .collect();

            let description_lines = if description_lines.is_empty() {
                vec![Line::from("No description available")]
            } else {
                description_lines
            };

            let style = if self.focused && self.selected_field == DetailField::Description {
                Style::default().bg(Color::Yellow)
            } else {
                Style::default()
            };

            let description_paragraph = Paragraph::new(description_lines)
                .wrap(Wrap { trim: true })
                .style(style);

            f.render_widget(description_paragraph, main_chunks[2]);
        }
    }

    fn render_subtasks_section(&self, f: &mut Frame, area: Rect) {
        let title = self.render_title_section("Subtasks", DetailField::Subtasks);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        f.render_widget(Paragraph::new(title), main_chunks[0]);

        let line_color = Color::Yellow;

        let separator = Line::from(vec![Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(line_color),
        )]);
        f.render_widget(Paragraph::new(separator), main_chunks[1]);

        if let Some(issue) = &self.issue {
            if issue.subtasks.is_empty() {
                // Show "No subtasks" message
                let no_subtasks_lines = vec![Line::from("No subtasks")];
                let style = if self.focused && self.selected_field == DetailField::Subtasks {
                    Style::default().bg(Color::Yellow)
                } else {
                    Style::default()
                };

                let no_subtasks_paragraph = Paragraph::new(no_subtasks_lines)
                    .wrap(Wrap { trim: true })
                    .style(style);

                f.render_widget(no_subtasks_paragraph, main_chunks[2]);
            } else {
                // Render subtasks as full issues (2-line format)
                let subtasks_items: Vec<ListItem> = issue
                    .subtasks
                    .iter()
                    .map(|subtask| {
                        self.issue_renderer.render_issue_ref(
                            subtask,
                            IssueItemMode::Full,
                            main_chunks[2].width,
                        )
                    })
                    .collect();

                let highlight_style =
                    if self.focused && self.selected_field == DetailField::Subtasks {
                        Style::default().bg(Color::Yellow)
                    } else {
                        Style::default()
                    };

                let subtasks_list = List::new(subtasks_items).highlight_style(highlight_style);

                f.render_widget(subtasks_list, main_chunks[2]);
            }
        }
    }

    pub fn set_issue(&mut self, issue: Option<JiraIssue>) {
        self.issue = issue;
        self.scroll_offset = 0;
        self.selected_field = DetailField::Key;
        self.edit_mode = false;
    }
}

#[cfg(test)]
mod tests;
