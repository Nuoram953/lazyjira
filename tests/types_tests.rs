use chrono::{TimeZone, Utc};
use lazyjira::services::types::*;
use serde_json::json;

#[test]
fn test_paginated_new() {
    let paginated: Paginated<String> = Paginated::new();
    assert_eq!(paginated.items.len(), 0);
    assert!(!paginated.has_more);
    assert_eq!(paginated.page, 1);
    assert_eq!(paginated.start_at, 0);
    assert_eq!(paginated.max_results, 25);
}

#[test]
fn test_paginated_default() {
    let paginated: Paginated<String> = Default::default();
    assert_eq!(paginated.items.len(), 0);
    assert!(!paginated.has_more);
    assert_eq!(paginated.page, 0);
    assert_eq!(paginated.start_at, 0);
    assert_eq!(paginated.max_results, 0);
}

#[test]
fn test_jira_issue_default() {
    let issue = JiraIssue::default();
    assert_eq!(issue.key, "");
    assert_eq!(issue.summary, "");
    assert!(issue.description.is_none());
    assert_eq!(issue.status, "");
    assert!(issue.priority.is_none());
    assert!(issue.assignee.is_none());
    assert!(issue.reporter.is_none());
    assert_eq!(issue.issue_type, "");
}

#[test]
fn test_jira_issue_from_api() {
    let api_issue = JiraIssueApi {
        key: "TEST-123".to_string(),
        fields: JiraIssueFields {
            summary: "Test issue".to_string(),
            description: Some(serde_json::Value::String("Test description".to_string())),
            status: JiraStatus {
                name: "In Progress".to_string(),
            },
            priority: Some(JiraPriority {
                name: "High".to_string(),
            }),
            assignee: Some(JiraUser {
                display_name: "John Doe".to_string(),
                email_address: Some("john@example.com".to_string()),
            }),
            reporter: Some(JiraUser {
                display_name: "Jane Smith".to_string(),
                email_address: Some("jane@example.com".to_string()),
            }),
            created: "2023-01-15T10:00:00.000Z".to_string(),
            updated: "2023-01-16T15:30:00.000Z".to_string(),
            issuetype: JiraIssueType {
                name: "Story".to_string(),
            },
        },
    };

    let issue: JiraIssue = api_issue.into();

    assert_eq!(issue.key, "TEST-123");
    assert_eq!(issue.summary, "Test issue");
    assert_eq!(
        issue.description,
        Some(serde_json::Value::String("Test description".to_string()))
    );
    assert_eq!(issue.status, "In Progress");
    assert_eq!(issue.priority, Some("High".to_string()));
    assert_eq!(issue.assignee, Some("John Doe".to_string()));
    assert_eq!(issue.reporter, Some("Jane Smith".to_string()));
    assert_eq!(issue.issue_type, "Story");

    // Test date parsing
    assert_eq!(
        issue.created,
        Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap()
    );
    assert_eq!(
        issue.updated,
        Utc.with_ymd_and_hms(2023, 1, 16, 15, 30, 0).unwrap()
    );
}

#[test]
fn test_jira_issue_from_api_with_none_values() {
    let api_issue = JiraIssueApi {
        key: "TEST-456".to_string(),
        fields: JiraIssueFields {
            summary: "Minimal issue".to_string(),
            description: None,
            status: JiraStatus {
                name: "Todo".to_string(),
            },
            priority: None,
            assignee: None,
            reporter: None,
            created: "2023-01-15T10:00:00.000Z".to_string(),
            updated: "2023-01-15T10:00:00.000Z".to_string(),
            issuetype: JiraIssueType {
                name: "Bug".to_string(),
            },
        },
    };

    let issue: JiraIssue = api_issue.into();

    assert_eq!(issue.key, "TEST-456");
    assert_eq!(issue.summary, "Minimal issue");
    assert!(issue.description.is_none());
    assert_eq!(issue.status, "Todo");
    assert!(issue.priority.is_none());
    assert!(issue.assignee.is_none());
    assert!(issue.reporter.is_none());
    assert_eq!(issue.issue_type, "Bug");
}

#[test]
fn test_jira_issue_from_api_invalid_dates() {
    let api_issue = JiraIssueApi {
        key: "TEST-000".to_string(),
        fields: JiraIssueFields {
            summary: "Invalid dates".to_string(),
            description: None,
            status: JiraStatus {
                name: "Todo".to_string(),
            },
            priority: None,
            assignee: None,
            reporter: None,
            created: "invalid-date".to_string(),
            updated: "also-invalid".to_string(),
            issuetype: JiraIssueType {
                name: "Bug".to_string(),
            },
        },
    };

    let issue: JiraIssue = api_issue.into();

    // Should use current time when dates are invalid
    let now = Utc::now();
    let diff_created = (issue.created - now).num_seconds().abs();
    let diff_updated = (issue.updated - now).num_seconds().abs();

    // Should be within a few seconds of current time
    assert!(diff_created < 5);
    assert!(diff_updated < 5);
}

#[test]
fn test_sprint_from_api() {
    let api_sprint = JiraSprintApi {
        id: 42,
        name: "Sprint 1".to_string(),
        state: "active".to_string(),
        start_date: Some("2023-01-15T09:00:00.000Z".to_string()),
        end_date: Some("2023-01-29T17:00:00.000Z".to_string()),
    };

    let sprint: Sprint = api_sprint.into();

    assert_eq!(sprint.id, "42");
    assert_eq!(sprint.name, "Sprint 1");
    assert_eq!(sprint.state, "active");
    assert_eq!(
        sprint.start_date,
        Some(Utc.with_ymd_and_hms(2023, 1, 15, 9, 0, 0).unwrap())
    );
    assert_eq!(
        sprint.end_date,
        Some(Utc.with_ymd_and_hms(2023, 1, 29, 17, 0, 0).unwrap())
    );
    assert!(sprint.issues.is_empty());
}

#[test]
fn test_sprint_from_api_no_dates() {
    let api_sprint = JiraSprintApi {
        id: 99,
        name: "Future Sprint".to_string(),
        state: "future".to_string(),
        start_date: None,
        end_date: None,
    };

    let sprint: Sprint = api_sprint.into();

    assert_eq!(sprint.id, "99");
    assert_eq!(sprint.name, "Future Sprint");
    assert_eq!(sprint.state, "future");
    assert!(sprint.start_date.is_none());
    assert!(sprint.end_date.is_none());
    assert!(sprint.issues.is_empty());
}

#[test]
fn test_sprint_from_api_invalid_dates() {
    let api_sprint = JiraSprintApi {
        id: 88,
        name: "Invalid Date Sprint".to_string(),
        state: "closed".to_string(),
        start_date: Some("not-a-date".to_string()),
        end_date: Some("also-not-a-date".to_string()),
    };

    let sprint: Sprint = api_sprint.into();

    assert_eq!(sprint.id, "88");
    assert!(sprint.start_date.is_none());
    assert!(sprint.end_date.is_none());
}

#[test]
fn test_jira_api_response_deserialization() {
    let json = json!({
        "issues": [
            {
                "key": "TEST-1",
                "fields": {
                    "summary": "Test issue 1",
                    "description": null,
                    "status": {"name": "Todo"},
                    "priority": null,
                    "assignee": null,
                    "reporter": null,
                    "created": "2023-01-15T10:00:00.000Z",
                    "updated": "2023-01-15T10:00:00.000Z",
                    "issuetype": {"name": "Bug"}
                }
            }
        ],
        "total": 1,
        "startAt": 0,
        "maxResults": 50
    });

    let response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(json).unwrap();

    assert!(response.issues.is_some());
    assert_eq!(response.issues.unwrap().len(), 1);
    assert_eq!(response.total, Some(1));
    assert_eq!(response.start_at, Some(0));
    assert_eq!(response.max_results, Some(50));
}

#[test]
fn test_jira_api_response_values_field() {
    let json = json!({
        "values": [
            {
                "id": 42,
                "name": "Test Sprint",
                "state": "active",
                "startDate": "2023-01-15T09:00:00.000Z",
                "endDate": "2023-01-29T17:00:00.000Z"
            }
        ]
    });

    let response: JiraApiResponse<JiraSprintApi> = serde_json::from_value(json).unwrap();

    assert!(response.values.is_some());
    assert_eq!(response.values.unwrap().len(), 1);
    assert!(response.issues.is_none());
}

#[test]
fn test_issue_serialization() {
    let issue = JiraIssue {
        key: "TEST-123".to_string(),
        summary: "Test issue".to_string(),
        description: Some(serde_json::Value::String("Test description".to_string())),
        status: "In Progress".to_string(),
        priority: Some("High".to_string()),
        assignee: Some("john@example.com".to_string()),
        reporter: Some("jane@example.com".to_string()),
        created: Utc.with_ymd_and_hms(2023, 1, 15, 10, 0, 0).unwrap(),
        updated: Utc.with_ymd_and_hms(2023, 1, 16, 15, 30, 0).unwrap(),
        issue_type: "Story".to_string(),
    };

    let serialized = serde_json::to_value(&issue).unwrap();
    let deserialized: JiraIssue = serde_json::from_value(serialized).unwrap();

    assert_eq!(deserialized.key, issue.key);
    assert_eq!(deserialized.summary, issue.summary);
    assert_eq!(deserialized.description, issue.description);
    assert_eq!(deserialized.status, issue.status);
    assert_eq!(deserialized.priority, issue.priority);
    assert_eq!(deserialized.assignee, issue.assignee);
    assert_eq!(deserialized.reporter, issue.reporter);
    assert_eq!(deserialized.created, issue.created);
    assert_eq!(deserialized.updated, issue.updated);
    assert_eq!(deserialized.issue_type, issue.issue_type);
}

#[test]
fn test_sprint_serialization() {
    let sprint = Sprint {
        id: "42".to_string(),
        name: "Test Sprint".to_string(),
        state: "active".to_string(),
        start_date: Some(Utc.with_ymd_and_hms(2023, 1, 15, 9, 0, 0).unwrap()),
        end_date: Some(Utc.with_ymd_and_hms(2023, 1, 29, 17, 0, 0).unwrap()),
        issues: vec![],
    };

    let serialized = serde_json::to_value(&sprint).unwrap();
    let deserialized: Sprint = serde_json::from_value(serialized).unwrap();

    assert_eq!(deserialized.id, sprint.id);
    assert_eq!(deserialized.name, sprint.name);
    assert_eq!(deserialized.state, sprint.state);
    assert_eq!(deserialized.start_date, sprint.start_date);
    assert_eq!(deserialized.end_date, sprint.end_date);
    assert_eq!(deserialized.issues.len(), 0);
}
