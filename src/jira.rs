use crate::models::{JiraIssue, SearchResponse, Sprint};
use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use std::env;

pub struct JiraClient {
    client: Client,
    base_url: String,
    username: String,
    api_token: String,
}

impl JiraClient {
    pub fn new(base_url: String, username: String, api_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            username,
            api_token,
        }
    }

    pub async fn get_issues_by_jql(&self, jql: &str, max_results: usize) -> Result<Vec<JiraIssue>> {
        let response: SearchResponse = self
            .client
            .get(format!("{}/rest/api/3/search/jql", &self.base_url))
            .query(&[("jql", jql), ("max_results", &max_results.to_string())])
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", response.issues);

        Ok(response.issues)
    }

    pub async fn get_issue(&self, issue_key: &str) -> Result<Option<JiraIssue>> {
        // TODO: Implement actual API call
        // Example endpoint: /rest/api/2/issue/{issue_key}
        Ok(None)
    }

    pub async fn get_recently_viewed_issues(&self, max_results: usize) -> Result<Vec<JiraIssue>> {
        // TODO: Implement actual API call
        // This might require using the activity stream or user history API
        Ok(vec![])
    }

    pub async fn get_recently_updated_issues(&self, max_results: usize) -> Result<Vec<JiraIssue>> {
        // Example JQL: "updated >= -7d ORDER BY updated DESC"
        self.get_issues_by_jql(
            "updated >= -7d&fields=*all ORDER BY updated DESC",
            max_results,
        )
        .await
    }

    fn build_auth_header(&self) -> String {
        use base64::prelude::*;
        let credentials = format!("{}:{}", self.username, self.api_token);
        format!("Basic {}", BASE64_STANDARD.encode(credentials.as_bytes()))
    }
}

// Configuration structure for Jira connection
#[derive(Debug, Clone)]
pub struct JiraConfig {
    pub base_url: String,
    pub username: String,
    pub api_token: String,
    pub board_id: Option<String>,
    pub max_results: usize,
}

impl Default for JiraConfig {
    fn default() -> Self {
        Self {
            base_url: "https://your-domain.atlassian.net".to_string(),
            username: "your-email@example.com".to_string(),
            api_token: "your-api-token".to_string(),
            board_id: None,
            max_results: 50,
        }
    }
}

impl JiraConfig {
    pub fn from_env() -> color_eyre::Result<Self> {
        Ok(Self {
            base_url: env::var("JIRA_BASE_URL")
                .unwrap_or_else(|_| "https://your-domain.atlassian.net".to_string()),
            username: env::var("JIRA_USERNAME")
                .unwrap_or_else(|_| "your-email@example.com".to_string()),
            api_token: env::var("JIRA_API_TOKEN").unwrap_or_else(|_| "your-api-token".to_string()),
            board_id: env::var("JIRA_BOARD_ID").ok(),
            max_results: env::var("JIRA_MAX_RESULTS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .unwrap_or(50),
        })
    }
}
