use crate::models::{AppData, JiraIssue};
use crate::navigation::{FocusedPane, NavigationState};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub struct UI;

impl UI {
    pub fn render(frame: &mut Frame, app_data: &AppData, nav_state: &NavigationState) {
        let size = frame.area();

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(0),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(size);

        match nav_state.current_view {
            crate::navigation::AppView::Main => {
                Self::render_main_view(frame, main_chunks[1], app_data, nav_state);
            }
            crate::navigation::AppView::Help => {
                Self::render_help_view(frame, main_chunks[1]);
            }
        }

        Self::render_footer(frame, main_chunks[2]);
    }

    fn render_footer(frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new(
            "q: Quit | ?: Help | hjkl: Navigate | Enter: Select & Focus Detail | Esc: Back",
        )
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, area);
    }

    fn render_main_view(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(area);

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(columns[0]);

        Self::render_sprint_section(frame, left_rows[0], app_data, nav_state);
        Self::render_last_updated_section(frame, left_rows[1], app_data, nav_state);
        Self::render_board_section(frame, left_rows[2], app_data, nav_state);

        Self::render_detail_section(frame, columns[1], app_data, nav_state);
    }

    fn render_sprint_section(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let is_focused = nav_state.focused_pane == FocusedPane::Sprint;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        if let Some(sprint) = &app_data.current_sprint {
            let items: Vec<ListItem> = sprint
                .issues
                .iter()
                .enumerate()
                .map(|(i, issue)| {
                    let style = if is_focused && i == nav_state.sprint_selected {
                        Style::default().bg(Color::DarkGray).fg(Color::White)
                    } else {
                        Style::default()
                    };
                    ListItem::new(format!(
                        "{}: {} [{}]",
                        issue.key, issue.summary, issue.status
                    ))
                    .style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{}", sprint.name))
                    .border_style(border_style),
            );
            frame.render_widget(list, area);
        } else {
            let paragraph = Paragraph::new("No active sprint").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Sprint")
                    .border_style(border_style),
            );
            frame.render_widget(paragraph, area);
        }
    }

    fn render_board_section(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let is_focused = nav_state.focused_pane == FocusedPane::Board;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let items: Vec<ListItem> = app_data
            .board_issues
            .iter()
            .enumerate()
            .map(|(i, issue)| {
                let style = if is_focused && i == nav_state.last_viewed_selected {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", issue.key, issue.summary)).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Issues")
                .border_style(border_style),
        );
        frame.render_widget(list, area);
    }

    fn render_last_updated_section(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let is_focused = nav_state.focused_pane == FocusedPane::LastUpdated;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let items: Vec<ListItem> = app_data
            .last_updated_issues
            .iter()
            .enumerate()
            .map(|(i, issue)| {
                let style = if is_focused && i == nav_state.last_updated_selected {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", issue.key, issue.summary)).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Last Updated Issues")
                .border_style(border_style),
        );
        frame.render_widget(list, area);
    }

    fn render_detail_section(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let is_focused = nav_state.focused_pane == FocusedPane::Detail;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        if let Some(issue) = &app_data.selected_issue {
            Self::render_issue_detail(frame, area, issue, border_style);
        } else {
            let paragraph = Paragraph::new(
                "No issue selected\n\nSelect an issue from the left panels to view details here.",
            )
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Issue Detail")
                    .border_style(border_style),
            );
            frame.render_widget(paragraph, area);
        }
    }

    fn render_issue_detail(frame: &mut Frame, area: Rect, issue: &JiraIssue, border_style: Style) {
        let content = vec![
            Line::from(vec![
                Span::styled(
                    "Key: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&issue.key),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Summary: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&issue.summary),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Status: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&issue.status, Self::status_color(&issue.status)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Type: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(&issue.issue_type),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Priority: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(issue.priority.as_deref().unwrap_or("None")),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Assignee: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(issue.assignee.as_deref().unwrap_or("Unassigned")),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Reporter: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(issue.reporter.as_deref().unwrap_or("Unknown")),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Created: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(issue.created.format("%Y-%m-%d %H:%M").to_string()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Updated: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(issue.updated.format("%Y-%m-%d %H:%M").to_string()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(
                issue
                    .description
                    .as_deref()
                    .unwrap_or("No description available"),
            ),
        ];

        let paragraph = Paragraph::new(content).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Issue Detail")
                .border_style(border_style),
        );
        frame.render_widget(paragraph, area);
    }

    fn render_help_view(frame: &mut Frame, area: Rect) {
        let content = vec![
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
        ];

        let paragraph = Paragraph::new(content).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Yellow)),
        );
        frame.render_widget(paragraph, area);
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
}
