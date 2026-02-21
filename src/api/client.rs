use super::endpoints::JiraEndpoints;
use super::error::{ApiError, ApiResult};
use super::types::*;
use log::info;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct JiraClient {
    client: Client,
    base_url: String,
    auth_header: String,
    endpoints: JiraEndpoints,
}

#[derive(Serialize)]
struct JiraQuery<'a> {
    jql: &'a str,
    fields: Option<&'a str>,
    max_results: Option<usize>,
}

impl JiraClient {
    pub fn new(base_url: String, username: String, api_token: String) -> ApiResult<Self> {
        let auth_header = Self::build_auth_header(&username, &api_token);

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ApiError::HttpClient(e.to_string()))?;

        let endpoints = JiraEndpoints::new(&base_url);

        Ok(Self {
            client,
            base_url,
            auth_header,
            endpoints,
        })
    }

    pub async fn get_issues_by_board_id(&self, board_id: &str) -> ApiResult<Vec<JiraIssue>> {
        let url = self.endpoints.board_issues(board_id);
        let query: Option<&JiraQuery> = None;
        let response = self.make_request(&url, Some(&query)).await?;

        let api_response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(response)
            .map_err(|e| {
                ApiError::Parse(format!("Failed to parse sprint issues response: {}", e))
            })?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        log::debug!("Found {} issues in board {}", issues.len(), board_id);
        for issue in &issues {
            log::trace!("  - {}: {} [{}]", issue.key, issue.summary, issue.status);
        }

        Ok(issues)
    }

    pub async fn get_current_sprint(&self, board_id: &str) -> ApiResult<Option<Sprint>> {
        let url = self.endpoints.active_sprint(board_id);
        let query: Option<&JiraQuery> = None;
        let response = self.make_request(&url, query).await?;

        let api_response: JiraApiResponse<JiraSprintApi> = serde_json::from_value(response)
            .map_err(|e| ApiError::Parse(format!("Failed to parse sprint response: {}", e)))?;

        if let Some(sprint_api) = api_response.values.unwrap_or_default().into_iter().next() {
            let sprint_id = &sprint_api.id.to_string();
            log::info!(
                "Found active sprint: {} ({})",
                sprint_api.name,
                sprint_api.state
            );

            let issues = match self.get_sprint_issues(sprint_id).await {
                Ok(issues) => {
                    log::info!(
                        "Found {} issues in sprint {}",
                        issues.len(),
                        sprint_api.name
                    );
                    issues
                }
                Err(e) => {
                    log::error!("Failed to fetch sprint issues: {}", e);
                    Vec::new()
                }
            };

            let mut sprint = Sprint::from(sprint_api);
            sprint.issues = issues;
            Ok(Some(sprint))
        } else {
            log::info!("No active sprint found");
            Ok(None)
        }
    }

    pub async fn get_sprint_issues(&self, sprint_id: &str) -> ApiResult<Vec<JiraIssue>> {
        let url = self.endpoints.sprint_issues(sprint_id);
        let query: Option<&JiraQuery> = None;
        let response = self.make_request(&url, query).await?;

        let api_response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(response)
            .map_err(|e| {
                ApiError::Parse(format!("Failed to parse sprint issues response: {}", e))
            })?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        log::debug!("Found {} issues in sprint {}", issues.len(), sprint_id);
        for issue in &issues {
            log::trace!("  - {}: {} [{}]", issue.key, issue.summary, issue.status);
        }

        Ok(issues)
    }

    pub async fn get_issues_by_jql(
        &self,
        jql: &str,
        max_results: usize,
    ) -> ApiResult<Vec<JiraIssue>> {
        let url = self.endpoints.search_issues(jql, max_results);
        let query = JiraQuery {
            jql: jql,
            fields: Some("*all"),
            max_results: Some(50),
        };
        let response = self.make_request(&url, Some(&query)).await?;

        let api_response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(response)
            .map_err(|e| ApiError::Parse(format!("Failed to parse search response: {}", e)))?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        info!("Found {} issues:", issues.len());
        for issue in &issues {
            log::debug!("  - {}: {} [{}]", issue.key, issue.summary, issue.status);
        }

        Ok(issues)
    }

    pub async fn get_issue(&self, issue_key: &str) -> ApiResult<Option<JiraIssue>> {
        let url = self.endpoints.issue_detail(issue_key);
        let query: Option<&JiraQuery> = None;
        let response = self.make_request(&url, query).await?;

        let api_issue: JiraIssueApi = serde_json::from_value(response)
            .map_err(|e| ApiError::Parse(format!("Failed to parse issue response: {}", e)))?;

        let issue = JiraIssue::from(api_issue);
        log::debug!(
            "Found issue: {}: {} [{}]",
            issue.key,
            issue.summary,
            issue.status
        );

        Ok(Some(issue))
    }

    pub async fn get_recently_viewed_issues(
        &self,
        max_results: usize,
    ) -> ApiResult<Vec<JiraIssue>> {
        let jql = "updated >= -30d ORDER BY updated DESC";
        self.get_issues_by_jql(jql, max_results).await
    }

    pub async fn get_recently_updated_issues(
        &self,
        max_results: usize,
    ) -> ApiResult<Vec<JiraIssue>> {
        let jql = "updated >= -7d ORDER BY updated DESC";
        self.get_issues_by_jql(jql, max_results).await
    }

    pub async fn get_board_issues(&self, board_id: &str) -> ApiResult<Vec<JiraIssue>> {
        self.get_issues_by_board_id(board_id).await
    }

    async fn make_request<T>(&self, url: &str, query: Option<&T>) -> ApiResult<Value>
    where
        T: Serialize,
    {
        let mut request = self
            .client
            .get(url)
            .header("Authorization", &self.auth_header)
            .header("Accept", "application/json");

        if let Some(q) = query {
            request = request.query(q);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ApiError::Http {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let json = response
            .json::<Value>()
            .await
            .map_err(|e| ApiError::Parse(e.to_string()))?;

        Ok(json)
    }

    fn build_auth_header(username: &str, api_token: &str) -> String {
        use base64::prelude::*;
        let credentials = format!("{}:{}", username, api_token);
        format!("Basic {}", BASE64_STANDARD.encode(credentials.as_bytes()))
    }
}
