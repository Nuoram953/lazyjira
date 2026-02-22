use lazyjira::{models::AppData, navigation::*, ui::UI};
use ratatui::{layout::Rect, style::Color, style::Style};

mod test_utils;
use test_utils::*;

#[test]
fn test_status_color() {
    assert_eq!(UI::status_color("To Do"), Style::default().fg(Color::Gray));
    assert_eq!(
        UI::status_color("In Progress"),
        Style::default().fg(Color::Yellow)
    );
    assert_eq!(UI::status_color("Done"), Style::default().fg(Color::Green));
    assert_eq!(UI::status_color("Blocked"), Style::default().fg(Color::Red));
    assert_eq!(
        UI::status_color("Unknown"),
        Style::default().fg(Color::White)
    );
}

#[test]
fn test_render_with_mock_data() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    let backend = terminal.backend();
    let buffer = backend.buffer();

    assert!(!buffer.area().is_empty());
}

#[test]
fn test_render_with_empty_data() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::new();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    let backend = terminal.backend();
    let buffer = backend.buffer();
    assert!(!buffer.area().is_empty());
}

#[test]
fn test_render_help_view() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::new();
    let mut nav_state = NavigationState::new();
    nav_state.current_view = AppView::Help;

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "LazyJira Help"));
    assert!(buffer_contains_text(&terminal, "Navigation"));
}

#[test]
fn test_render_with_different_focused_panes() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();

    let panes = [
        FocusedPane::Sprint,
        FocusedPane::LastUpdated,
        FocusedPane::Board,
        FocusedPane::Detail,
    ];

    for pane in panes {
        let mut nav_state = NavigationState::new();
        nav_state.focused_pane = pane;

        terminal
            .draw(|frame| {
                UI::render(frame, &app_data, &nav_state);
            })
            .unwrap();

        let backend = terminal.backend();
        let buffer = backend.buffer();
        assert!(!buffer.area().is_empty());
    }
}

#[test]
fn test_render_footer_content() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::new();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "Quit"));
    assert!(buffer_contains_text(&terminal, "Help"));
    assert!(buffer_contains_text(&terminal, "Navigate"));
}

#[test]
fn test_render_sprint_section_with_data() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "PROJ-123"));
    assert!(buffer_contains_text(
        &terminal,
        "Implement user authentication"
    ));
    assert!(buffer_contains_text(&terminal, "Sprint 1"));
}

#[test]
fn test_render_sprint_section_without_data() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::new();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "No active sprint"));
}

#[test]
fn test_render_board_and_last_updated_sections() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "Issues"));

    assert!(buffer_contains_text(&terminal, "Last Updated Issues"));
}

#[test]
fn test_render_detail_section_with_selected_issue() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "Issue Detail"));
    assert!(buffer_contains_text(&terminal, "Key:"));
    assert!(buffer_contains_text(&terminal, "Summary:"));
    assert!(buffer_contains_text(&terminal, "Status:"));
    assert!(buffer_contains_text(&terminal, "PROJ-123"));
}

#[test]
fn test_render_detail_section_without_selected_issue() {
    let mut terminal = create_test_terminal();
    let mut app_data = AppData::with_mock_data();
    app_data.selected_issue = None;
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "No issue selected"));
}

#[test]
fn test_render_with_loading_state() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::loading_for_test();
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    let backend = terminal.backend();
    let buffer = backend.buffer();
    assert!(!buffer.area().is_empty());
}

#[test]
fn test_render_with_many_issues() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_n_issues_for_test(10);
    let nav_state = NavigationState::new();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state);
        })
        .unwrap();

    assert!(buffer_contains_text(&terminal, "TEST-1"));
    assert!(buffer_contains_text(&terminal, "TEST-2"));

    let test_count = (1..=5)
        .filter(|i| buffer_contains_text(&terminal, &format!("TEST-{}", i)))
        .count();
    assert!(test_count >= 2, "Should display at least 2 test issues");
}

#[test]
fn test_render_focuses_correctly() {
    let mut terminal = create_test_terminal();
    let app_data = AppData::with_mock_data();

    let mut nav_state1 = NavigationState::new();
    nav_state1.focused_pane = FocusedPane::Sprint;

    let mut nav_state2 = NavigationState::new();
    nav_state2.focused_pane = FocusedPane::Board;

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state1);
        })
        .unwrap();
    let buffer1 = terminal.backend().buffer().clone();

    terminal
        .draw(|frame| {
            UI::render(frame, &app_data, &nav_state2);
        })
        .unwrap();
    let buffer2 = terminal.backend().buffer().clone();

    assert_eq!(buffer1.area(), buffer2.area());
}

#[test]
fn test_ui_layout_constraints() {
    let app_data = AppData::with_mock_data();
    let nav_state = NavigationState::new();

    let sizes = [(40, 20), (120, 30), (80, 24)];

    for (width, height) in sizes {
        let mut terminal = create_test_terminal_with_size(width, height);

        terminal
            .draw(|frame| {
                UI::render(frame, &app_data, &nav_state);
            })
            .unwrap();

        let backend = terminal.backend();
        let buffer = backend.buffer();
        assert_eq!(*buffer.area(), Rect::new(0, 0, width, height));
    }
}
