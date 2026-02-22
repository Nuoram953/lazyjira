use super::adf::AdfDocument;
use chrono::Utc;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub use crate::models::{JiraIssue, Sprint};

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
    #[serde(deserialize_with = "deserialize_description")]
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
    #[serde(flatten)]
    pub adf: Option<AdfDocument>,

    pub content: Option<String>,
}

fn deserialize_description<'de, D>(deserializer: D) -> Result<Option<JiraDescription>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;

    match value {
        None => Ok(None),
        Some(Value::String(text)) => Ok(Some(JiraDescription {
            adf: None,
            content: Some(text),
        })),
        Some(Value::Object(_)) => match serde_json::from_value::<AdfDocument>(value.unwrap()) {
            Ok(adf) => Ok(Some(JiraDescription {
                adf: Some(adf),
                content: None,
            })),
            Err(_) => Ok(Some(JiraDescription {
                adf: None,
                content: Some("Unknown format".to_string()),
            })),
        },
        Some(_) => Ok(Some(JiraDescription {
            adf: None,
            content: Some(value.unwrap().to_string()),
        })),
    }
}

impl JiraDescription {
    pub fn to_text(&self) -> Option<String> {
        if let Some(adf) = &self.adf {
            let text = adf.to_formatted_text();
            if !text.trim().is_empty() {
                return Some(text);
            }
        }

        self.content.clone()
    }

    pub fn to_plain_text(&self) -> Option<String> {
        if let Some(adf) = &self.adf {
            let text = adf.to_plain_text();
            if !text.trim().is_empty() {
                return Some(text);
            }
        }

        self.content.clone()
    }
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

#[derive(Debug, Deserialize)]
pub struct JiraTransitionsResponse {
    pub transitions: Vec<JiraTransition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JiraTransition {
    pub id: String,
    pub name: String,
    pub to: JiraStatus,
}

#[derive(Debug, serde::Serialize)]
pub struct JiraTransitionRequest {
    pub transition: JiraTransitionRequestTransition,
}

#[derive(Debug, serde::Serialize)]
pub struct JiraTransitionRequestTransition {
    pub id: String,
}

impl From<JiraIssueApi> for JiraIssue {
    fn from(api_issue: JiraIssueApi) -> Self {
        Self {
            key: api_issue.key,
            summary: api_issue.fields.summary,
            description: api_issue.fields.description.and_then(|d| d.to_text()),
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
            issues: Vec::new(),
        }
    }
}
