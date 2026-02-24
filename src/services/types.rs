use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub page: usize,
    #[allow(dead_code)]
    pub start_at: usize,
    #[allow(dead_code)]
    pub max_results: usize,
}

impl<T> Default for Paginated<T> {
    fn default() -> Self {
        Self {
            items: vec![],
            has_more: false,
            page: 0,
            start_at: 0,
            max_results: 0,
        }
    }
}

impl<T> Paginated<T> {
    pub fn new() -> Self {
        Self {
            items: vec![],
            has_more: false,
            page: 1,
            start_at: 0,
            max_results: 25,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub summary: String,
    pub description: Option<serde_json::Value>,
    pub status: String,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub issue_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Sprint {
    pub id: String,
    pub name: String,
    pub state: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub issues: Vec<JiraIssue>,
}

#[derive(Debug, Deserialize)]
pub struct JiraApiResponse<T> {
    pub issues: Option<Vec<T>>,
    pub values: Option<Vec<T>>,
    #[allow(dead_code)]
    pub total: Option<u32>,
    #[serde(rename = "startAt")]
    #[allow(dead_code)]
    pub start_at: Option<u32>,
    #[serde(rename = "maxResults")]
    #[allow(dead_code)]
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
    pub description: Option<serde_json::Value>,
    pub status: JiraStatus,
    pub priority: Option<JiraPriority>,
    pub assignee: Option<JiraUser>,
    pub reporter: Option<JiraUser>,
    pub created: String,
    pub updated: String,
    pub issuetype: JiraIssueType,
}

#[derive(Debug, Deserialize, Clone)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    #[allow(dead_code)]
    pub end_date: Option<String>,
}

impl From<JiraIssueApi> for JiraIssue {
    fn from(api_issue: JiraIssueApi) -> Self {
        Self {
            key: api_issue.key,
            summary: api_issue.fields.summary,
            description: api_issue.fields.description,
            status: api_issue.fields.status.name,
            priority: api_issue.fields.priority.map(|p| p.name),
            assignee: api_issue.fields.assignee.map(|u| u.display_name),
            reporter: api_issue.fields.reporter.map(|u| u.display_name),
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
            start_date: api_sprint.start_date.and_then(|d| d.parse().ok()),
            end_date: api_sprint.end_date.and_then(|d| d.parse().ok()),
            issues: vec![],
        }
    }
}
