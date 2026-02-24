use std::env;

#[derive(Debug, Clone)]
pub struct JiraConfig {
    pub base_url: String,
    pub username: String,
    pub api_token: String,
    pub board_id: String,
    pub max_results: usize,
}

impl Default for JiraConfig {
    fn default() -> Self {
        Self {
            base_url: "https://your-domain.atlassian.net".to_string(),
            username: "your-email@example.com".to_string(),
            api_token: "your-api-token".to_string(),
            board_id: "1".to_string(),
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
            board_id: env::var("JIRA_BOARD_ID").unwrap_or_else(|_| "1".to_string()),
            max_results: env::var("JIRA_MAX_RESULTS")
                .unwrap_or_else(|_| "25".to_string())
                .parse()
                .unwrap_or(25),
        })
    }
}
