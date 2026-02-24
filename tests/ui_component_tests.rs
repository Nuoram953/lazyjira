use chrono::{DateTime, Utc};
use lazyjira::services::sort::SortMode;
use lazyjira::services::types::JiraIssue;
use lazyjira::ui::components::{IssueList, ListAction};

fn create_test_issue(key: &str, summary: &str) -> JiraIssue {
    JiraIssue {
        key: key.to_string(),
        summary: summary.to_string(),
        description: None,
        status: "Todo".to_string(),
        priority: Some("Medium".to_string()),
        assignee: None,
        reporter: None,
        created: Utc::now(),
        updated: Utc::now(),
        issue_type: "Story".to_string(),
    }
}

fn create_test_issue_with_time(key: &str, summary: &str, updated: DateTime<Utc>) -> JiraIssue {
    JiraIssue {
        key: key.to_string(),
        summary: summary.to_string(),
        description: None,
        status: "Todo".to_string(),
        priority: Some("Medium".to_string()),
        assignee: None,
        reporter: None,
        created: updated,
        updated,
        issue_type: "Story".to_string(),
    }
}

/// Helper function to mock JQL refetch behavior - sorts issues based on the given sort mode
fn mock_jql_sort(mut issues: Vec<JiraIssue>, sort_mode: SortMode) -> Vec<JiraIssue> {
    match sort_mode {
        SortMode::KeyAsc => {
            issues.sort_by(|a, b| a.key.cmp(&b.key));
        }
        SortMode::KeyDesc => {
            issues.sort_by(|a, b| b.key.cmp(&a.key));
        }
        SortMode::UpdatedAsc => {
            issues.sort_by(|a, b| a.updated.cmp(&b.updated));
        }
        SortMode::UpdatedDesc => {
            issues.sort_by(|a, b| b.updated.cmp(&a.updated));
        }
        SortMode::PriorityAsc | SortMode::PriorityDesc => {
            // For this test, just fall back to key sorting since priority logic is more complex
            issues.sort_by(|a, b| a.key.cmp(&b.key));
        }
    }
    issues
}

#[test]
fn test_issue_list_new() {
    let list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    assert_eq!(list.title, "Test List");
    assert_eq!(list.result.items.len(), 0);
    assert!(!list.result.has_more);
    assert!(!list.focused);
    assert_eq!(list.sort_mode, SortMode::KeyAsc);
    assert!(!list.is_loading);

    assert_eq!(list.state_selected(), None);
}

#[test]
fn test_sort_mode_next() {
    assert_eq!(SortMode::KeyAsc.next(), SortMode::KeyDesc);
    assert_eq!(SortMode::KeyDesc.next(), SortMode::UpdatedAsc);
}

#[test]
fn test_sort_mode_label() {
    assert_eq!(SortMode::KeyAsc.label(), "Key ↑");
    assert_eq!(SortMode::KeyDesc.label(), "Key ↓");
}

#[test]
fn test_move_up_empty_list() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);

    list.move_up();
    assert_eq!(list.state_selected(), None);
}

#[test]
fn test_move_up_with_items() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.result.items = vec![
        create_test_issue("TEST-1", "First issue"),
        create_test_issue("TEST-2", "Second issue"),
    ];

    list.move_up();
    assert_eq!(list.state_selected(), Some(0));

    list.move_up();
    assert_eq!(list.state_selected(), Some(0));

    list.select_item(Some(1));
    list.move_up();
    assert_eq!(list.state_selected(), Some(0));
}

#[test]
fn test_move_down_empty_list() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);

    let action = list.move_down();
    assert_eq!(action, ListAction::None);
    assert_eq!(list.state_selected(), None);
}

#[test]
fn test_move_down_with_items() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.result.items = vec![
        create_test_issue("TEST-1", "First issue"),
        create_test_issue("TEST-2", "Second issue"),
    ];

    let action = list.move_down();
    assert_eq!(action, ListAction::None);
    assert_eq!(list.state_selected(), Some(0));

    let action = list.move_down();
    assert_eq!(action, ListAction::None);
    assert_eq!(list.state_selected(), Some(1));

    let action = list.move_down();
    assert_eq!(action, ListAction::None);
    assert_eq!(list.state_selected(), Some(1));
}

#[test]
fn test_move_down_with_pagination() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.result.items = vec![
        create_test_issue("TEST-1", "First issue"),
        create_test_issue("TEST-2", "Second issue"),
    ];
    list.result.has_more = true;
    list.select_item(Some(1));

    let action = list.move_down();
    assert_eq!(action, ListAction::RequestMore);
}

#[test]
fn test_move_down_loading_state() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.result.items = vec![create_test_issue("TEST-1", "First issue")];
    list.result.has_more = true;
    list.is_loading = true;
    list.select_item(Some(0));

    let action = list.move_down();
    assert_eq!(action, ListAction::None);
}

#[test]
fn test_sort_items_empty_list() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.sort_mode = SortMode::KeyAsc;

    list.sort_items();

    assert_eq!(list.result.items.len(), 0);
    assert_eq!(list.state_selected(), None);
}

#[test]
fn test_cycle_sort() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);

    // Test that cycle_sort() properly cycles through sort modes and returns Sort action
    assert_eq!(list.sort_mode, SortMode::KeyAsc);

    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::KeyDesc);

    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::UpdatedAsc);

    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::UpdatedDesc);
}

#[test]
fn test_cycle_sort_with_mock_refetch() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);

    // Initial unsorted items
    list.result.items = vec![
        create_test_issue("ZZZ-1", "Last issue"),
        create_test_issue("AAA-1", "First issue"),
        create_test_issue("MMM-1", "Middle issue"),
    ];

    // Test initial state
    assert_eq!(list.sort_mode, SortMode::KeyAsc);
    assert_eq!(list.result.items[0].key, "ZZZ-1");

    // Cycle sort and mock the JQL refetch behavior
    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::KeyDesc);

    // Mock the result of JQL refetch with KeyDesc sorting
    let sorted_items_desc = vec![
        create_test_issue("ZZZ-1", "Last issue"),
        create_test_issue("MMM-1", "Middle issue"),
        create_test_issue("AAA-1", "First issue"),
    ];
    list.result.items = sorted_items_desc;
    assert_eq!(list.result.items[0].key, "ZZZ-1");

    // Cycle sort again and mock the next refetch
    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::UpdatedAsc);

    // Mock the result of JQL refetch with UpdatedAsc sorting
    // (In this test, we'll just simulate by key since we don't have real updated dates)
    let sorted_items_updated_asc = vec![
        create_test_issue("AAA-1", "First issue"),
        create_test_issue("MMM-1", "Middle issue"),
        create_test_issue("ZZZ-1", "Last issue"),
    ];
    list.result.items = sorted_items_updated_asc;
    assert_eq!(list.result.items[0].key, "AAA-1");
}

#[test]
fn test_cycle_sort_complete_workflow_with_helper() {
    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);

    // Create test issues with different timestamps for proper sorting
    let base_time = Utc::now();
    let initial_issues = vec![
        create_test_issue_with_time(
            "ZZZ-1",
            "Last issue",
            base_time - chrono::Duration::hours(4),
        ), // Oldest
        create_test_issue_with_time(
            "AAA-1",
            "First issue",
            base_time - chrono::Duration::hours(1),
        ), // Newest
        create_test_issue_with_time(
            "MMM-1",
            "Middle issue",
            base_time - chrono::Duration::hours(2),
        ), // Middle
        create_test_issue_with_time(
            "BBB-1",
            "Second issue",
            base_time - chrono::Duration::hours(3),
        ), // Second oldest
    ];
    list.result.items = initial_issues.clone();

    // Test cycle through different sort modes with proper mocking
    for expected_sort_mode in [
        SortMode::KeyDesc,
        SortMode::UpdatedAsc,
        SortMode::UpdatedDesc,
    ] {
        let action = list.cycle_sort();
        assert_eq!(action, ListAction::Sort);
        assert_eq!(list.sort_mode, expected_sort_mode);

        // Mock the JQL refetch behavior
        let sorted_issues = mock_jql_sort(initial_issues.clone(), expected_sort_mode);
        list.result.items = sorted_issues;

        // Verify the mock sorting worked correctly
        match expected_sort_mode {
            SortMode::KeyDesc => {
                // Should be sorted by key descending: ZZZ-1, MMM-1, BBB-1, AAA-1
                assert_eq!(list.result.items[0].key, "ZZZ-1");
                assert_eq!(list.result.items[1].key, "MMM-1");
                assert_eq!(list.result.items[2].key, "BBB-1");
                assert_eq!(list.result.items[3].key, "AAA-1");
            }
            SortMode::UpdatedAsc => {
                // Should be sorted by updated time ascending: oldest first
                // ZZZ-1 (4h ago), BBB-1 (3h ago), MMM-1 (2h ago), AAA-1 (1h ago)
                assert_eq!(list.result.items[0].key, "ZZZ-1");
                assert_eq!(list.result.items[1].key, "BBB-1");
                assert_eq!(list.result.items[2].key, "MMM-1");
                assert_eq!(list.result.items[3].key, "AAA-1");
            }
            SortMode::UpdatedDesc => {
                // Should be sorted by updated time descending: newest first
                // AAA-1 (1h ago), MMM-1 (2h ago), BBB-1 (3h ago), ZZZ-1 (4h ago)
                assert_eq!(list.result.items[0].key, "AAA-1");
                assert_eq!(list.result.items[1].key, "MMM-1");
                assert_eq!(list.result.items[2].key, "BBB-1");
                assert_eq!(list.result.items[3].key, "ZZZ-1");
            }
            _ => {}
        }
    }
}

#[test]
fn test_list_action_equality() {
    assert_eq!(ListAction::None, ListAction::None);
    assert_eq!(ListAction::RequestMore, ListAction::RequestMore);
    assert_ne!(ListAction::None, ListAction::RequestMore);
}

#[test]
fn test_spinner_animation_during_loading() {
    use ratatui::{backend::TestBackend, Terminal};

    let mut list = IssueList::new("Test".to_string(), false, SortMode::KeyAsc);
    list.is_loading = true;

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    for _ in 0..20 {
        terminal
            .draw(|f| {
                let area = f.area();
                list.draw(f, area, true);
            })
            .unwrap();
    }

    assert!(list.is_loading);
}
