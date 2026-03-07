use super::*;
use crate::services::types::{JiraIssue, JiraIssueRef};
use chrono::{DateTime, Utc};
use ratatui::{backend::TestBackend, layout::Rect, Terminal};
use serde_json::json;

/// Helper function to create a test JiraIssue
fn create_test_issue() -> JiraIssue {
    JiraIssue {
        key: "TEST-123".to_string(),
        summary: "Test issue summary".to_string(),
        description: Some(json!({
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "This is a test description."
                        }
                    ]
                }
            ]
        })),
        status: "To Do".to_string(),
        priority: Some("High".to_string()),
        assignee: Some("John Doe".to_string()),
        reporter: Some("Jane Smith".to_string()),
        created: DateTime::parse_from_rfc3339("2024-01-01T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        updated: DateTime::parse_from_rfc3339("2024-01-02T15:30:00Z")
            .unwrap()
            .with_timezone(&Utc),
        issue_type: "Bug".to_string(),
        parent: None,
        subtasks: vec![
            JiraIssueRef {
                key: "TEST-124".to_string(),
                summary: Some("Subtask 1".to_string()),
                issue_type: Some("Sub-task".to_string()),
            },
            JiraIssueRef {
                key: "TEST-125".to_string(),
                summary: Some("Subtask 2".to_string()),
                issue_type: Some("Sub-task".to_string()),
            },
        ],
    }
}

fn create_test_terminal() -> Terminal<TestBackend> {
    let backend = TestBackend::new(80, 24);
    Terminal::new(backend).unwrap()
}

// DetailField enum tests
#[test]
fn test_detail_field_next() {
    assert_eq!(DetailField::Key.next(), DetailField::Summary);
    assert_eq!(DetailField::Summary.next(), DetailField::Details);
    assert_eq!(DetailField::Details.next(), DetailField::Description);
    assert_eq!(DetailField::Description.next(), DetailField::Parent);
    assert_eq!(DetailField::Parent.next(), DetailField::Subtasks);
    assert_eq!(DetailField::Subtasks.next(), DetailField::LinkedItems);
    assert_eq!(DetailField::LinkedItems.next(), DetailField::Key);
}

#[test]
fn test_detail_field_prev() {
    assert_eq!(DetailField::Key.prev(), DetailField::LinkedItems);
    assert_eq!(DetailField::Summary.prev(), DetailField::Key);
    assert_eq!(DetailField::Details.prev(), DetailField::Summary);
    assert_eq!(DetailField::Description.prev(), DetailField::Details);
    assert_eq!(DetailField::Parent.prev(), DetailField::Description);
    assert_eq!(DetailField::Subtasks.prev(), DetailField::Parent);
    assert_eq!(DetailField::LinkedItems.prev(), DetailField::Subtasks);
}

#[test]
fn test_detail_field_cycling() {
    let mut field = DetailField::Key;

    // Test complete cycle forward
    for _ in 0..7 {
        field = field.next();
    }
    assert_eq!(field, DetailField::Key);

    // Test complete cycle backward
    for _ in 0..7 {
        field = field.prev();
    }
    assert_eq!(field, DetailField::Key);
}

// IssueDetail struct tests
#[test]
fn test_issue_detail_new() {
    let detail = IssueDetail::new();

    assert_eq!(detail.issue, None);
    assert_eq!(detail.scroll_offset, 0);
    assert!(!detail.focused);
    assert_eq!(detail.selected_field, DetailField::Key);
    assert!(!detail.edit_mode);
}

#[test]
fn test_move_up_down() {
    let mut detail = IssueDetail::new();
    assert_eq!(detail.selected_field, DetailField::Key);

    detail.move_down();
    assert_eq!(detail.selected_field, DetailField::Summary);

    detail.move_down();
    assert_eq!(detail.selected_field, DetailField::Details);

    detail.move_up();
    assert_eq!(detail.selected_field, DetailField::Summary);

    detail.move_up();
    assert_eq!(detail.selected_field, DetailField::Key);
}

#[test]
fn test_edit_mode() {
    let mut detail = IssueDetail::new();
    assert!(!detail.edit_mode);

    detail.enter_edit_mode();
    assert!(detail.edit_mode);

    detail.enter_edit_mode(); // Should remain true
    assert!(detail.edit_mode);

    detail.exit_edit_mode();
    assert!(!detail.edit_mode);

    detail.exit_edit_mode(); // Should remain false
    assert!(!detail.edit_mode);
}

#[test]
fn test_set_issue() {
    let mut detail = IssueDetail::new();
    let test_issue = create_test_issue();

    // Set some non-default values first
    detail.scroll_offset = 10;
    detail.selected_field = DetailField::Details;
    detail.edit_mode = true;

    detail.set_issue(Some(test_issue.clone()));

    assert_eq!(detail.issue, Some(test_issue));
    assert_eq!(detail.scroll_offset, 0); // Should reset
    assert_eq!(detail.selected_field, DetailField::Key); // Should reset
    assert!(!detail.edit_mode); // Should reset
}

#[test]
fn test_set_issue_none() {
    let mut detail = IssueDetail::new();
    let test_issue = create_test_issue();
    detail.set_issue(Some(test_issue));

    detail.set_issue(None);

    assert_eq!(detail.issue, None);
    assert_eq!(detail.scroll_offset, 0);
    assert_eq!(detail.selected_field, DetailField::Key);
    assert!(!detail.edit_mode);
}

// Rendering tests
#[test]
fn test_draw_with_no_issue() {
    let mut detail = IssueDetail::new();
    let mut terminal = create_test_terminal();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            detail.draw(f, area, false);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let rendered_text: String = buffer.content().iter().map(|cell| cell.symbol()).collect();

    // Should contain "No issue selected" somewhere
    assert!(
        rendered_text.contains("No issue selected"),
        "Should display 'No issue selected' when no issue is set. Actual content: {}",
        rendered_text.chars().take(200).collect::<String>()
    );
}

#[test]
fn test_draw_with_issue() {
    let mut detail = IssueDetail::new();
    let test_issue = create_test_issue();
    detail.set_issue(Some(test_issue));

    let mut terminal = create_test_terminal();

    terminal
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            detail.draw(f, area, false);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let rendered_text: String = buffer.content().iter().map(|cell| cell.symbol()).collect();

    // Check that issue data is rendered
    assert!(
        rendered_text.contains("TEST-123"),
        "Should display issue key"
    );
    assert!(
        rendered_text.contains("Test issue summary"),
        "Should display issue summary"
    );
    assert!(rendered_text.contains("Bug"), "Should display issue type");
    assert!(rendered_text.contains("To Do"), "Should display status");
}

#[test]
fn test_draw_focused_vs_unfocused() {
    let mut detail = IssueDetail::new();
    let test_issue = create_test_issue();
    detail.set_issue(Some(test_issue));

    let mut terminal_focused = create_test_terminal();
    let mut terminal_unfocused = create_test_terminal();

    // Draw focused
    terminal_focused
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            detail.draw(f, area, true);
        })
        .unwrap();

    // Draw unfocused
    terminal_unfocused
        .draw(|f| {
            let area = Rect::new(0, 0, 80, 24);
            detail.draw(f, area, false);
        })
        .unwrap();

    let focused_buffer = terminal_focused.backend().buffer();
    let unfocused_buffer = terminal_unfocused.backend().buffer();

    // The buffers should be different (different border colors)
    assert_ne!(
        focused_buffer.content(),
        unfocused_buffer.content(),
        "Focused and unfocused rendering should be different"
    );
}

#[test]
fn test_navigation_cycle() {
    let mut detail = IssueDetail::new();
    let test_issue = create_test_issue();
    detail.set_issue(Some(test_issue));
    detail.focused = true;

    let mut terminal = create_test_terminal();

    // Test navigation through different fields
    for _ in 0..7 {
        // Navigate through all fields once
        terminal
            .draw(|f| {
                let area = Rect::new(0, 0, 80, 24);
                detail.draw(f, area, true);
            })
            .unwrap();

        detail.move_down();
    }

    // Should be back to Key after full cycle
    assert_eq!(detail.selected_field, DetailField::Key);
}
