pub mod components;

use crate::api::types::JiraTransition;
use crate::models::AppData;
use crate::navigation::{AppView, NavigationState};
use components::*;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct UI;

impl UI {
    pub fn render(frame: &mut Frame, app_data: &AppData, nav_state: &NavigationState) {
        Self::render_with_transitions(frame, app_data, nav_state, &[])
    }

    pub fn render_with_transitions(
        frame: &mut Frame,
        app_data: &AppData,
        nav_state: &NavigationState,
        transitions: &[JiraTransition],
    ) {
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
            AppView::Main => {
                Self::render_main_layout(frame, main_chunks[1], app_data, nav_state);
            }
            AppView::Help => {
                help_view::HelpView::new().render(frame, main_chunks[1]);
            }
            AppView::TransitionSelector => {
                Self::render_main_layout(frame, main_chunks[1], app_data, nav_state);

                transition_selector::TransitionSelector::new(transitions, nav_state, app_data)
                    .render(frame, main_chunks[1]);
            }
        }

        footer::Footer::new().render(frame, main_chunks[2]);
    }

    fn render_main_layout(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let main_columns = Layout::default()
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
            .split(main_columns[0]);

        Self::render_sprint_list(frame, left_rows[0], app_data, nav_state);
        Self::render_last_updated_list(frame, left_rows[1], app_data, nav_state);
        Self::render_board_list(frame, left_rows[2], app_data, nav_state);

        issue_detail::IssueDetail::new(app_data, nav_state).render(frame, main_columns[1]);
    }

    fn render_sprint_list(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let issues = if let Some(sprint) = &app_data.current_sprint {
            sprint.issues.as_slice()
        } else {
            &[]
        };

        let title = if let Some(sprint) = &app_data.current_sprint {
            sprint.name.to_string()
        } else {
            "Sprint: No active sprint".to_string()
        };

        issue_list::IssueList::new(
            issues,
            &title,
            nav_state.sprint_selected,
            nav_state.focused_pane == crate::navigation::FocusedPane::Sprint,
        )
        .render(frame, area);
    }

    fn render_last_updated_list(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let title = "Last Updated";

        issue_list::IssueList::new(
            &app_data.last_updated_issues,
            title,
            nav_state.last_updated_selected,
            nav_state.focused_pane == crate::navigation::FocusedPane::LastUpdated,
        )
        .render(frame, area);
    }

    fn render_board_list(
        frame: &mut Frame,
        area: Rect,
        app_data: &AppData,
        nav_state: &NavigationState,
    ) {
        let title = "Board Issues";

        issue_list::IssueList::new(
            &app_data.board_issues,
            title,
            nav_state.last_viewed_selected,
            nav_state.focused_pane == crate::navigation::FocusedPane::Board,
        )
        .render(frame, area);
    }
}
