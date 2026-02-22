use lazyjira::models::AppData;
use lazyjira::navigation::{AppView, Direction, FocusedPane, NavigationState};
use lazyjira::ui::UI;

mod test_utils;
use test_utils::*;

#[test]
fn test_app_navigation_workflow() {
    let mut app_data = AppData::with_mock_data();
    let mut nav_state = NavigationState::new();
    let mut terminal = create_test_terminal();

    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);
    assert_eq!(nav_state.sprint_selected, 0);

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    nav_state.move_selection(Direction::Down, app_data.board_issues.len());
    assert_eq!(nav_state.last_viewed_selected, 1);

    if let Some(selected_issue) = app_data.board_issues.get(nav_state.last_viewed_selected) {
        app_data.selected_issue = Some(selected_issue.clone());
    }

    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);
    assert_eq!(nav_state.previous_pane, Some(FocusedPane::Board));

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();
}

#[test]
fn test_navigation_boundaries() {
    let app_data = AppData::with_n_issues_for_test(3);
    let mut nav_state = NavigationState::new();

    nav_state.focused_pane = FocusedPane::Sprint;

    nav_state.move_selection(
        Direction::Down,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    nav_state.move_selection(
        Direction::Down,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    nav_state.move_selection(
        Direction::Down,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    assert_eq!(nav_state.sprint_selected, 2);

    nav_state.move_selection(
        Direction::Down,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    assert_eq!(nav_state.sprint_selected, 2);

    nav_state.move_selection(
        Direction::Up,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    assert_eq!(nav_state.sprint_selected, 1);

    nav_state.move_selection(
        Direction::Up,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    nav_state.move_selection(
        Direction::Up,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );
    assert_eq!(nav_state.sprint_selected, 0);
}

#[test]
fn test_app_state_scenarios() {
    let mut terminal = create_test_terminal();
    let nav_state = NavigationState::new();

    let empty_data = AppData::new();
    terminal
        .draw(|frame| {
            UI::render(frame, &empty_data, &nav_state);
        })
        .unwrap();

    let loading_data = AppData::loading_for_test();
    terminal
        .draw(|frame| {
            UI::render(frame, &loading_data, &nav_state);
        })
        .unwrap();

    let large_data = AppData::with_n_issues_for_test(50);
    terminal
        .draw(|frame| {
            UI::render(frame, &large_data, &nav_state);
        })
        .unwrap();

    // Test passes if rendering doesn't panic
}

#[test]
fn test_view_switching() {
    let app_data = AppData::with_mock_data();
    let mut nav_state = NavigationState::new();
    let mut terminal = create_test_terminal();

    assert_eq!(nav_state.current_view, AppView::Main);
    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.current_view = AppView::Help;
    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.current_view = AppView::Main;
    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();
}

#[test]
fn test_issue_selection_sync() {
    let mut app_data = AppData::with_mock_data();
    let mut nav_state = NavigationState::new();

    nav_state.focused_pane = FocusedPane::Sprint;
    nav_state.move_selection(
        Direction::Down,
        app_data.current_sprint.as_ref().unwrap().issues.len(),
    );

    if let Some(sprint) = &app_data.current_sprint {
        if let Some(selected_issue) = sprint.issues.get(nav_state.sprint_selected) {
            app_data.selected_issue = Some(selected_issue.clone());
            assert_eq!(
                app_data.selected_issue.as_ref().unwrap().key,
                sprint.issues[nav_state.sprint_selected].key
            );
        }
    }

    nav_state.focused_pane = FocusedPane::Board;
    nav_state.move_selection(Direction::Down, app_data.board_issues.len());

    if let Some(selected_issue) = app_data.board_issues.get(nav_state.last_viewed_selected) {
        app_data.selected_issue = Some(selected_issue.clone());
        assert_eq!(
            app_data.selected_issue.as_ref().unwrap().key,
            app_data.board_issues[nav_state.last_viewed_selected].key
        );
    }

    nav_state.focused_pane = FocusedPane::LastUpdated;
    nav_state.move_selection(Direction::Down, app_data.last_updated_issues.len());

    if let Some(selected_issue) = app_data
        .last_updated_issues
        .get(nav_state.last_updated_selected)
    {
        app_data.selected_issue = Some(selected_issue.clone());
        assert_eq!(
            app_data.selected_issue.as_ref().unwrap().key,
            app_data.last_updated_issues[nav_state.last_updated_selected].key
        );
    }
}

#[test]
fn test_complete_user_flow() {
    let mut app_data = AppData::with_n_issues_for_test(5);
    let mut nav_state = NavigationState::new();
    let mut terminal = create_test_terminal_with_size(100, 30);

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    for _ in 0..3 {
        nav_state.move_selection(
            Direction::Down,
            app_data.current_sprint.as_ref().unwrap().issues.len(),
        );
    }
    assert_eq!(nav_state.sprint_selected, 3);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_selection(Direction::Down, app_data.last_updated_issues.len());
    if let Some(selected_issue) = app_data
        .last_updated_issues
        .get(nav_state.last_updated_selected)
    {
        app_data.selected_issue = Some(selected_issue.clone());
    }

    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    nav_state.current_view = AppView::Help;
    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    nav_state.current_view = AppView::Main;
    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(app_data.selected_issue.is_some());
}

#[test]
fn test_error_conditions() {
    let mut terminal = create_test_terminal();
    let mut nav_state = NavigationState::new();

    let empty_data = AppData::new();
    terminal
        .draw(|frame| {
            UI::render(frame, &empty_data, &nav_state);
        })
        .unwrap();

    nav_state.move_selection(Direction::Down, 0);
    nav_state.move_selection(Direction::Up, 0);
    nav_state.move_focus(Direction::Right);
    nav_state.move_focus(Direction::Left);

    let mut data_no_selection = AppData::with_mock_data();
    data_no_selection.selected_issue = None;
    terminal
        .draw(|frame| {
            UI::render(frame, &data_no_selection, &nav_state);
        })
        .unwrap();

    nav_state.focus_detail();
    terminal
        .draw(|frame| {
            UI::render(frame, &data_no_selection, &nav_state);
        })
        .unwrap();
}
