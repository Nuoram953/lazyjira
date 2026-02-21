use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub issues: Vec<JiraIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub summary: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub issue_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: String,
    pub name: String,
    pub state: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub issues: Vec<JiraIssue>,
}

#[derive(Debug, Clone)]
pub struct AppData {
    pub current_sprint: Option<Sprint>,
    pub board_issues: Vec<JiraIssue>,
    pub last_updated_issues: Vec<JiraIssue>,
    pub selected_issue: Option<JiraIssue>,
    pub loading: bool,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            current_sprint: None,
            board_issues: Vec::new(),
            last_updated_issues: Vec::new(),
            selected_issue: None,
            loading: false,
            last_updated: None,
        }
    }

    pub fn with_mock_data() -> Self {
        let mock_issues = vec![
            JiraIssue {
                key: "PROJ-123".to_string(),
                summary: "Implement user authentication".to_string(),
                description: Some("Add OAuth 2.0 authentication for the application".to_string()),
                status: "In Progress".to_string(),
                priority: Some("High".to_string()),
                assignee: Some("john.doe@example.com".to_string()),
                reporter: Some("jane.smith@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Story".to_string(),
            },
            JiraIssue {
                key: "PROJ-124".to_string(),
                summary: "Fix login bug on mobile devices".to_string(),
                description: Some("Users cannot log in on mobile Safari".to_string()),
                status: "To Do".to_string(),
                priority: Some("Medium".to_string()),
                assignee: Some("alice.johnson@example.com".to_string()),
                reporter: Some("bob.wilson@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Bug".to_string(),
            },
            JiraIssue {
                key: "PROJ-125".to_string(),
                summary: "Update API documentation".to_string(),
                description: Some("Add examples for new endpoints".to_string()),
                status: "Done".to_string(),
                priority: Some("Low".to_string()),
                assignee: Some("charlie.brown@example.com".to_string()),
                reporter: Some("diana.prince@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Task".to_string(),
            },
        ];

        let current_sprint = Some(Sprint {
            id: "SP-001".to_string(),
            name: "Sprint 1".to_string(),
            state: "Active".to_string(),
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now()),
            issues: mock_issues.clone(),
        });

        Self {
            current_sprint,
            board_issues: mock_issues[0..2].to_vec(),
            last_updated_issues: mock_issues[1..3].to_vec(),
            selected_issue: Some(mock_issues[0].clone()),
            loading: false,
            last_updated: Some(Utc::now()),
        }
    }
}
