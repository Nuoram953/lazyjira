use chrono::{DateTime, Utc};
use lazyjira::services::sort::SortMode;
use lazyjira::services::types::JiraIssue;
use lazyjira::ui::components::{IssueList, JqlTab, ListAction, TabAction};

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

    list.result.items = vec![
        create_test_issue("ZZZ-1", "Last issue"),
        create_test_issue("AAA-1", "First issue"),
        create_test_issue("MMM-1", "Middle issue"),
    ];

    assert_eq!(list.sort_mode, SortMode::KeyAsc);
    assert_eq!(list.result.items[0].key, "ZZZ-1");

    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::KeyDesc);

    let sorted_items_desc = vec![
        create_test_issue("ZZZ-1", "Last issue"),
        create_test_issue("MMM-1", "Middle issue"),
        create_test_issue("AAA-1", "First issue"),
    ];
    list.result.items = sorted_items_desc;
    assert_eq!(list.result.items[0].key, "ZZZ-1");

    let action = list.cycle_sort();
    assert_eq!(action, ListAction::Sort);
    assert_eq!(list.sort_mode, SortMode::UpdatedAsc);

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

    let base_time = Utc::now();
    let initial_issues = vec![
        create_test_issue_with_time(
            "ZZZ-1",
            "Last issue",
            base_time - chrono::Duration::hours(4),
        ),
        create_test_issue_with_time(
            "AAA-1",
            "First issue",
            base_time - chrono::Duration::hours(1),
        ),
        create_test_issue_with_time(
            "MMM-1",
            "Middle issue",
            base_time - chrono::Duration::hours(2),
        ),
        create_test_issue_with_time(
            "BBB-1",
            "Second issue",
            base_time - chrono::Duration::hours(3),
        ),
    ];
    list.result.items = initial_issues.clone();

    for expected_sort_mode in [
        SortMode::KeyDesc,
        SortMode::UpdatedAsc,
        SortMode::UpdatedDesc,
    ] {
        let action = list.cycle_sort();
        assert_eq!(action, ListAction::Sort);
        assert_eq!(list.sort_mode, expected_sort_mode);

        let sorted_issues = mock_jql_sort(initial_issues.clone(), expected_sort_mode);
        list.result.items = sorted_issues;

        match expected_sort_mode {
            SortMode::KeyDesc => {
                assert_eq!(list.result.items[0].key, "ZZZ-1");
                assert_eq!(list.result.items[1].key, "MMM-1");
                assert_eq!(list.result.items[2].key, "BBB-1");
                assert_eq!(list.result.items[3].key, "AAA-1");
            }
            SortMode::UpdatedAsc => {
                assert_eq!(list.result.items[0].key, "ZZZ-1");
                assert_eq!(list.result.items[1].key, "BBB-1");
                assert_eq!(list.result.items[2].key, "MMM-1");
                assert_eq!(list.result.items[3].key, "AAA-1");
            }
            SortMode::UpdatedDesc => {
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

#[test]
fn test_jql_tab_creation() {
    let tab = JqlTab::new("Test Tab", "assignee = currentUser()");

    assert_eq!(tab.name, "Test Tab");
    assert_eq!(tab.jql, "assignee = currentUser()");
    assert_eq!(tab.description, None);
}

#[test]
fn test_jql_tab_with_description() {
    let tab = JqlTab::new("Test Tab", "assignee = currentUser()")
        .with_description("Issues assigned to me");

    assert_eq!(tab.name, "Test Tab");
    assert_eq!(tab.jql, "assignee = currentUser()");
    assert_eq!(tab.description, Some("Issues assigned to me".to_string()));
}

#[test]
fn test_issue_list_without_tabs() {
    let list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    assert!(!list.tabs_enabled);
    assert_eq!(list.tabs().len(), 0);
    assert_eq!(list.current_tab(), None);
    assert_eq!(list.current_jql(), None);
}

#[test]
fn test_issue_list_with_tabs() {
    let tabs = vec![
        JqlTab::new("All", ""),
        JqlTab::new("Assigned", "assignee = currentUser()"),
        JqlTab::new("Recent", "updated >= -7d"),
    ];

    let list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    assert!(list.tabs_enabled);
    assert_eq!(list.tabs().len(), 3);

    let current_tab = list.current_tab().unwrap();
    assert_eq!(current_tab.name, "All");
    assert_eq!(current_tab.jql, "");

    assert_eq!(list.current_jql(), Some("".to_string()));
}

#[test]
fn test_enable_disable_tabs() {
    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    assert!(!list.tabs_enabled);

    list.enable_tabs();
    assert!(list.tabs_enabled);
    assert!(!list.tabs().is_empty());

    list.disable_tabs();
    assert!(!list.tabs_enabled);
}

#[test]
fn test_add_tab_to_empty_list() {
    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    assert!(!list.tabs_enabled);
    assert_eq!(list.tabs().len(), 0);

    let tab = JqlTab::new("Custom", "project = TEST");
    list.add_tab(tab);

    assert!(list.tabs_enabled);
    assert_eq!(list.tabs().len(), 1);
    assert_eq!(list.current_tab().unwrap().name, "Custom");
}

#[test]
fn test_add_tab_to_existing_tabs() {
    let tabs = vec![
        JqlTab::new("All", ""),
        JqlTab::new("Assigned", "assignee = currentUser()"),
    ];

    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    assert_eq!(list.tabs().len(), 2);

    let new_tab = JqlTab::new("High Priority", "priority in (Highest, High)");
    list.add_tab(new_tab);

    assert_eq!(list.tabs().len(), 3);
    assert_eq!(list.tabs()[2].name, "High Priority");
}

#[test]
fn test_move_tab_left() {
    let tabs = vec![
        JqlTab::new("All", ""),
        JqlTab::new("Assigned", "assignee = currentUser()"),
        JqlTab::new("Recent", "updated >= -7d"),
    ];

    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    assert_eq!(list.current_tab().unwrap().name, "All");

    let action = list.move_tab_left();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "Recent");

    let action = list.move_tab_left();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "Assigned");

    let action = list.move_tab_left();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "All");
}

#[test]
fn test_move_tab_right() {
    let tabs = vec![
        JqlTab::new("All", ""),
        JqlTab::new("Assigned", "assignee = currentUser()"),
        JqlTab::new("Recent", "updated >= -7d"),
    ];

    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    assert_eq!(list.current_tab().unwrap().name, "All");

    let action = list.move_tab_right();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "Assigned");

    let action = list.move_tab_right();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "Recent");

    let action = list.move_tab_right();
    assert_eq!(action, TabAction::TabChanged);
    assert_eq!(list.current_tab().unwrap().name, "All");
}

#[test]
fn test_move_tabs_without_tabs_enabled() {
    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    let action = list.move_tab_left();
    assert_eq!(action, TabAction::NoAction);

    let action = list.move_tab_right();
    assert_eq!(action, TabAction::NoAction);
}

#[test]
fn test_single_tab_movement() {
    let tabs = vec![JqlTab::new("Only Tab", "project = TEST")];

    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    let action = list.move_tab_left();
    assert_eq!(action, TabAction::NoAction);

    let action = list.move_tab_right();
    assert_eq!(action, TabAction::NoAction);

    assert_eq!(list.current_tab().unwrap().name, "Only Tab");
}

#[test]
fn test_default_tabs() {
    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc);

    list.enable_tabs();

    let tabs = list.tabs();
    assert!(tabs.len() >= 6);

    assert_eq!(tabs[0].name, "All");
    assert_eq!(tabs[0].jql, "");

    assert_eq!(tabs[1].name, "Assigned");
    assert_eq!(tabs[1].jql, "assignee = currentUser()");

    assert_eq!(tabs[2].name, "Recent");
    assert_eq!(tabs[2].jql, "updated >= -7d");
}

#[test]
fn test_current_jql_returns_correct_values() {
    let tabs = vec![
        JqlTab::new("All", ""),
        JqlTab::new("Assigned", "assignee = currentUser()"),
        JqlTab::new("High Priority", "priority in (Highest, High)"),
    ];

    let mut list = IssueList::new("Test List".to_string(), false, SortMode::KeyAsc).with_tabs(tabs);

    assert_eq!(list.current_jql(), Some("".to_string()));

    list.move_tab_right();
    assert_eq!(
        list.current_jql(),
        Some("assignee = currentUser()".to_string())
    );

    list.move_tab_right();
    assert_eq!(
        list.current_jql(),
        Some("priority in (Highest, High)".to_string())
    );
}

#[test]
fn test_tab_action_enum() {
    assert_ne!(TabAction::TabChanged, TabAction::NoAction);

    let action = TabAction::TabChanged;
    assert!(format!("{:?}", action).contains("TabChanged"));
}

#[test]
fn test_jql_tab_equality() {
    let tab1 = JqlTab::new("Test", "assignee = currentUser()");
    let tab2 = JqlTab::new("Test", "assignee = currentUser()");
    let tab3 = JqlTab::new("Different", "assignee = currentUser()");

    assert_eq!(tab1, tab2);
    assert_ne!(tab1, tab3);
}

#[test]
fn test_jql_tab_clone() {
    let original =
        JqlTab::new("Test", "assignee = currentUser()").with_description("Test description");

    let cloned = original.clone();

    assert_eq!(original, cloned);
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.jql, cloned.jql);
    assert_eq!(original.description, cloned.description);
}
