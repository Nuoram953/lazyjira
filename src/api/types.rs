use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Re-export from main models for convenience
pub use crate::models::{JiraIssue, Sprint};

// API-specific types that differ from your app models
#[derive(Debug, Deserialize)]
pub struct JiraApiResponse<T> {
    pub issues: Option<Vec<T>>,
    pub values: Option<Vec<T>>,
    pub total: Option<u32>,
    #[serde(rename = "startAt")]
    pub start_at: Option<u32>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueApi {
    pub key: String,
    pub fields: JiraIssueFields,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueFields {
    pub summary: String,
    pub description: Option<JiraDescription>,
    pub status: JiraStatus,
    pub priority: Option<JiraPriority>,
    pub assignee: Option<JiraUser>,
    pub reporter: Option<JiraUser>,
    pub created: String,
    pub updated: String,
    pub issuetype: JiraIssueType,
}

#[derive(Debug, Deserialize)]
pub struct JiraDescription {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JiraStatus {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraPriority {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraUser {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JiraIssueType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct JiraSprintApi {
    pub id: u32,
    pub name: String,
    pub state: String,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

// Conversion functions from API types to app types
impl From<JiraIssueApi> for JiraIssue {
    fn from(api_issue: JiraIssueApi) -> Self {
        Self {
            key: api_issue.key,
            summary: api_issue.fields.summary,
            description: api_issue.fields.description.and_then(|d| d.content),
            status: api_issue.fields.status.name,
            priority: api_issue.fields.priority.map(|p| p.name),
            assignee: api_issue.fields.assignee.and_then(|u| u.email_address),
            reporter: api_issue.fields.reporter.and_then(|u| u.email_address),
            created: api_issue
                .fields
                .created
                .parse()
                .unwrap_or_else(|_| Utc::now()),
            updated: api_issue
                .fields
                .updated
                .parse()
                .unwrap_or_else(|_| Utc::now()),
            issue_type: api_issue.fields.issuetype.name,
        }
    }
}

impl From<JiraSprintApi> for Sprint {
    fn from(api_sprint: JiraSprintApi) -> Self {
        Self {
            id: api_sprint.id.to_string(),
            name: api_sprint.name,
            state: api_sprint.state,
            start_date: api_sprint.start_date.and_then(|s| s.parse().ok()),
            end_date: api_sprint.end_date.and_then(|s| s.parse().ok()),
            issues: Vec::new(), // Sprint issues would need a separate API call
        }
    }
}
