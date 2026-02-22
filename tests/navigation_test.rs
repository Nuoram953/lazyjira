use lazyjira::navigation::*;

#[test]
fn test_navigation_state_new() {
    let nav_state = NavigationState::new();

    assert_eq!(nav_state.current_view, AppView::Main);
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);
    assert!(nav_state.previous_pane.is_none());
    assert_eq!(nav_state.sprint_selected, 0);
    assert_eq!(nav_state.last_viewed_selected, 0);
    assert_eq!(nav_state.last_updated_selected, 0);
}

#[test]
fn test_move_focus_left_right() {
    let mut nav_state = NavigationState::new();

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    nav_state.move_focus(Direction::Left);
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_focus(Direction::Left);
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);

    nav_state.move_focus(Direction::Left);
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);
}

#[test]
fn test_move_selection_in_sprint() {
    let mut nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::Sprint;

    nav_state.move_selection(Direction::Down, 3);
    assert_eq!(nav_state.sprint_selected, 1);

    nav_state.move_selection(Direction::Down, 3);
    assert_eq!(nav_state.sprint_selected, 2);

    nav_state.move_selection(Direction::Down, 3);
    assert_eq!(nav_state.sprint_selected, 2);

    nav_state.move_selection(Direction::Up, 3);
    assert_eq!(nav_state.sprint_selected, 1);

    nav_state.move_selection(Direction::Up, 3);
    assert_eq!(nav_state.sprint_selected, 0);

    nav_state.move_selection(Direction::Up, 3);
    assert_eq!(nav_state.sprint_selected, 0);
}

#[test]
fn test_move_selection_in_board() {
    let mut nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::Board;

    nav_state.move_selection(Direction::Down, 2);
    assert_eq!(nav_state.last_viewed_selected, 1);

    nav_state.move_selection(Direction::Down, 2);
    assert_eq!(nav_state.last_viewed_selected, 1);

    nav_state.move_selection(Direction::Up, 2);
    assert_eq!(nav_state.last_viewed_selected, 0);
}

#[test]
fn test_move_selection_in_last_updated() {
    let mut nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::LastUpdated;

    nav_state.move_selection(Direction::Down, 2);
    assert_eq!(nav_state.last_updated_selected, 1);

    nav_state.move_selection(Direction::Down, 2);
    assert_eq!(nav_state.last_updated_selected, 1);

    nav_state.move_selection(Direction::Up, 2);
    assert_eq!(nav_state.last_updated_selected, 0);
}

#[test]
fn test_focus_detail_from_different_panes() {
    let mut nav_state = NavigationState::new();

    nav_state.focused_pane = FocusedPane::Sprint;
    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);
    assert_eq!(nav_state.previous_pane, Some(FocusedPane::Sprint));

    nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::Board;
    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);
    assert_eq!(nav_state.previous_pane, Some(FocusedPane::Board));

    nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::LastUpdated;
    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);
    assert_eq!(nav_state.previous_pane, Some(FocusedPane::LastUpdated));
}

#[test]
fn test_go_back_from_detail() {
    let mut nav_state = NavigationState::new();

    nav_state.focused_pane = FocusedPane::Sprint;
    nav_state.focus_detail();

    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);

    nav_state.focused_pane = FocusedPane::Detail;
    nav_state.previous_pane = None;
    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);
}

#[test]
fn test_go_back_from_non_detail_pane() {
    let mut nav_state = NavigationState::new();
    nav_state.focused_pane = FocusedPane::Sprint;

    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);
}

#[test]
fn test_selection_boundaries_with_empty_list() {
    let mut nav_state = NavigationState::new();

    nav_state.move_selection(Direction::Down, 0);
    assert_eq!(nav_state.sprint_selected, 0);

    nav_state.move_selection(Direction::Up, 0);
    assert_eq!(nav_state.sprint_selected, 0);
}

#[test]
fn test_navigation_state_full_workflow() {
    let mut nav_state = NavigationState::new();

    assert_eq!(nav_state.focused_pane, FocusedPane::Sprint);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::LastUpdated);

    nav_state.move_focus(Direction::Right);
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);

    nav_state.move_selection(Direction::Down, 3);
    nav_state.focus_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Detail);
    assert_eq!(nav_state.previous_pane, Some(FocusedPane::Board));

    nav_state.go_back_from_detail();
    assert_eq!(nav_state.focused_pane, FocusedPane::Board);
    assert_eq!(nav_state.last_viewed_selected, 1);
}
