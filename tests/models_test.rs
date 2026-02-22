use chrono::Utc;
use lazyjira::models::*;

#[test]
fn test_app_data_new() {
    let app_data = AppData::new();

    assert!(app_data.current_sprint.is_none());
    assert!(app_data.board_issues.is_empty());
    assert!(app_data.last_updated_issues.is_empty());
    assert!(app_data.selected_issue.is_none());
    assert!(!app_data.loading);
    assert!(app_data.last_updated.is_none());
}

#[test]
fn test_app_data_with_mock_data() {
    let app_data = AppData::with_mock_data();

    assert!(app_data.current_sprint.is_some());
    assert_eq!(app_data.board_issues.len(), 2);
    assert_eq!(app_data.last_updated_issues.len(), 2);
    assert!(app_data.selected_issue.is_some());
    assert!(!app_data.loading);
    assert!(app_data.last_updated.is_some());

    let sprint = app_data.current_sprint.unwrap();
    assert_eq!(sprint.id, "SP-001");
    assert_eq!(sprint.name, "Sprint 1");
    assert_eq!(sprint.state, "Active");
    assert_eq!(sprint.issues.len(), 3);
}

#[test]
fn test_app_data_loading_for_test() {
    let app_data = AppData::loading_for_test();
    assert!(app_data.loading);
    assert!(app_data.current_sprint.is_none());
}

#[test]
fn test_app_data_with_n_issues() {
    let app_data = AppData::with_n_issues_for_test(5);

    assert_eq!(app_data.board_issues.len(), 5);
    assert_eq!(app_data.last_updated_issues.len(), 5);
    assert!(app_data.selected_issue.is_some());

    if let Some(sprint) = app_data.current_sprint {
        assert_eq!(sprint.issues.len(), 5);
    }

    let statuses: Vec<&str> = app_data
        .board_issues
        .iter()
        .map(|i| i.status.as_str())
        .collect();
    assert!(statuses.contains(&"To Do"));
    assert!(statuses.contains(&"In Progress"));
    assert!(statuses.contains(&"Done"));
}

#[test]
fn test_jira_issue_serialization() {
    let issue = JiraIssue {
        key: "TEST-1".to_string(),
        summary: "Test issue".to_string(),
        description: Some("Test description".to_string()),
        status: "In Progress".to_string(),
        priority: Some("High".to_string()),
        assignee: Some("test@example.com".to_string()),
        reporter: Some("reporter@example.com".to_string()),
        created: Utc::now(),
        updated: Utc::now(),
        issue_type: "Story".to_string(),
    };

    let serialized = serde_json::to_string(&issue).unwrap();
    let deserialized: JiraIssue = serde_json::from_str(&serialized).unwrap();

    assert_eq!(issue.key, deserialized.key);
    assert_eq!(issue.summary, deserialized.summary);
    assert_eq!(issue.status, deserialized.status);
}

#[test]
fn test_sprint_with_issues() {
    let issues = vec![JiraIssue {
        key: "SPRINT-1".to_string(),
        summary: "Sprint issue 1".to_string(),
        description: None,
        status: "To Do".to_string(),
        priority: None,
        assignee: None,
        reporter: None,
        created: Utc::now(),
        updated: Utc::now(),
        issue_type: "Task".to_string(),
    }];

    let sprint = Sprint {
        id: "SP-TEST".to_string(),
        name: "Test Sprint".to_string(),
        state: "Active".to_string(),
        start_date: Some(Utc::now()),
        end_date: None,
        issues,
    };

    assert_eq!(sprint.issues.len(), 1);
    assert_eq!(sprint.issues[0].key, "SPRINT-1");
}
