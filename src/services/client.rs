use super::types::*;
use crate::services::{sort::SortMode, JiraConfig, JiraEndpoints};

use super::error::{ApiError, ApiResult};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::time::Duration;

#[derive(Serialize)]
pub struct SprintIssuesQuery {
    pub jql: String,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "startAt")]
    pub start_at: Option<usize>,
}

#[derive(Serialize)]
pub struct RecentlyUpdatedIssuesQuery<'a> {
    pub jql: String,
    #[serde(rename = "maxResults")]
    pub max_results: Option<usize>,
    #[serde(rename = "startAt")]
    pub start_at: Option<usize>,
    pub fields: &'a str,
}

#[derive(Debug, Clone)]
pub struct JiraClient {
    client: Client,
    pub config: JiraConfig,
    auth_header: String,
    endpoints: JiraEndpoints,
}

impl JiraClient {
    pub fn new() -> ApiResult<Self> {
        let config = JiraConfig::from_env().map_err(|e| ApiError::Config(e.to_string()))?;

        let auth_header = Self::build_auth_header(&config.username, &config.api_token);

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ApiError::HttpClient(e.to_string()))?;

        let endpoints = JiraEndpoints::new(&config.base_url);

        Ok(Self {
            client,
            auth_header,
            endpoints,
            config,
        })
    }

    pub fn build_auth_header(username: &str, api_token: &str) -> String {
        use base64::prelude::*;
        let credentials = format!("{}:{}", username, api_token);
        format!("Basic {}", BASE64_STANDARD.encode(credentials.as_bytes()))
    }

    pub async fn make_request<Q>(&self, url: &str, query: Option<&Q>) -> ApiResult<Value>
    where
        Q: Serialize + ?Sized,
    {
        let mut request = self
            .client
            .get(url)
            .header("Authorization", &self.auth_header);

        if let Some(q) = query {
            request = request.query(q);
        }

        log::debug!("Making HTTP request to {}", url);

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;

        log::debug!("Got HTTP response: {}", response.status());

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

    pub async fn fetch_active_sprint(&self) -> ApiResult<Option<JiraSprintApi>> {
        let url = self.endpoints.active_sprint(&self.config.board_id);

        let response = self.make_request(&url, None::<&()>).await?;

        let api_response: JiraApiResponse<JiraSprintApi> = serde_json::from_value(response)
            .map_err(|e| ApiError::Parse(format!("Failed to parse sprint response: {}", e)))?;

        let sprint = api_response.values.unwrap_or_default().into_iter().next();

        Ok(sprint)
    }

    pub async fn fetch_sprint_issues(
        &self,
        sprint_id: &str,
        sort: SortMode,
        page: usize,
    ) -> ApiResult<Paginated<JiraIssue>> {
        let url = self
            .endpoints
            .get_all_issues_for_sprint(&self.config.board_id, sprint_id);

        let start_at = page * self.config.max_results;

        let jql = sort.jql_order_by().to_string();

        let query = SprintIssuesQuery {
            jql,
            max_results: Some(self.config.max_results),
            start_at: Some(start_at),
        };

        let response = self.make_request(&url, Some(&query)).await?;

        let api_response: JiraApiResponse<JiraIssueApi> =
            serde_json::from_value(response).map_err(|e| ApiError::Parse(e.to_string()))?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        let start_at = api_response.start_at.unwrap_or(0);
        let max_results = api_response
            .max_results
            .unwrap_or(self.config.max_results as u32);
        let total = api_response.total.unwrap_or(0);

        let has_more = start_at + max_results < total;

        Ok(Paginated {
            items: issues,
            has_more,
            page,
            start_at: start_at as usize,
            max_results: max_results as usize,
        })
    }

    pub async fn fetch_current_sprint_issues(
        &self,
        sort: SortMode,
        page: usize,
    ) -> ApiResult<Paginated<JiraIssue>> {
        let Some(sprint_api) = self.fetch_active_sprint().await? else {
            log::info!("No active sprint found");
            return Ok(Paginated::new());
        };

        log::info!(
            "Found active sprint: {} ({})",
            sprint_api.name,
            sprint_api.state
        );

        let sprint_id = sprint_api.id.to_string();

        self.fetch_sprint_issues(&sprint_id, sort, page).await
    }

    pub async fn fetch_recently_updated_issues(
        &self,
        sort: SortMode,
        page: usize,
    ) -> ApiResult<Paginated<JiraIssue>> {
        let url = self.endpoints.search_issues();

        let start_at = page * self.config.max_results;

        let jql = format!("updated >= -30d {}", sort.jql_order_by());

        let query = RecentlyUpdatedIssuesQuery {
            jql,
            max_results: Some(self.config.max_results),
            start_at: Some(start_at),
            fields: "*all",
        };

        let response = self.make_request(&url, Some(&query)).await?;

        let api_response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(response)
            .map_err(|e| {
                log::error!("Parse error: {:?}", e);
                ApiError::Parse(format!("Failed to parse search response: {}", e))
            })?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        let start_at = api_response.start_at.unwrap_or(0);
        let max_results = api_response
            .max_results
            .unwrap_or(self.config.max_results as u32);
        let total = api_response.total.unwrap_or(0);

        let has_more = start_at + max_results < total;

        Ok(Paginated {
            items: issues,
            has_more,
            page,
            start_at: start_at as usize,
            max_results: max_results as usize,
        })
    }

    pub async fn fetch_backlog_issues(
        &self,
        sort: SortMode,
        page: usize,
    ) -> ApiResult<Paginated<JiraIssue>> {
        let url = self.endpoints.backlog_issues(&self.config.board_id);

        let start_at = page * self.config.max_results;

        let jql = sort.jql_order_by().to_string();

        let query = SprintIssuesQuery {
            jql,
            max_results: Some(self.config.max_results),
            start_at: Some(start_at),
        };

        let response = self.make_request(&url, Some(&query)).await?;

        let api_response: JiraApiResponse<JiraIssueApi> = serde_json::from_value(response)
            .map_err(|e| {
                log::error!("Parse error: {:?}", e);
                ApiError::Parse(format!("Failed to parse search response: {}", e))
            })?;

        let issues: Vec<JiraIssue> = api_response
            .issues
            .unwrap_or_default()
            .into_iter()
            .map(JiraIssue::from)
            .collect();

        let start_at = api_response.start_at.unwrap_or(0);
        let max_results = api_response
            .max_results
            .unwrap_or(self.config.max_results as u32);
        let total = api_response.total.unwrap_or(0);

        let has_more = start_at + max_results < total;

        Ok(Paginated {
            items: issues,
            has_more,
            page,
            start_at: start_at as usize,
            max_results: max_results as usize,
        })
    }
}
