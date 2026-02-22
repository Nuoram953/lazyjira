use crate::api::{error::ApiResult, JiraClient};
use crate::models::{AppData, JiraIssue, Sprint};
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct DataManager {
    client: JiraClient,
    data: Arc<RwLock<AppData>>,
    config: DataManagerConfig,
}

#[derive(Debug, Clone)]
pub struct DataManagerConfig {
    pub board_id: Option<String>,
    pub max_results: usize,
    pub auto_refresh_interval: Option<std::time::Duration>,
}

impl Default for DataManagerConfig {
    fn default() -> Self {
        Self {
            board_id: None,
            max_results: 50,
            auto_refresh_interval: Some(std::time::Duration::from_secs(300)),
        }
    }
}

impl DataManager {
    pub fn new(client: JiraClient, config: DataManagerConfig) -> Self {
        Self {
            client,
            data: Arc::new(RwLock::new(AppData::new())),
            config,
        }
    }

    pub async fn get_data(&self) -> AppData {
        self.data.read().await.clone()
    }

    pub async fn refresh_all_data(&self) -> ApiResult<()> {
        info!("Starting data refresh...");

        debug!("Board ID configured: {:?}", self.config.board_id);
        debug!("Max results: {}", self.config.max_results);

        let (sprint_result, last_updated_result, board_result) = tokio::join!(
            self.fetch_current_sprint(),
            self.fetch_last_updated_issues(),
            self.fetch_board_issues()
        );

        let mut data = self.data.write().await;

        match sprint_result {
            Ok(sprint) => {
                data.current_sprint = sprint;
                if let Some(ref sprint) = data.current_sprint {
                    info!(
                        "Successfully loaded sprint: {} with {} issues",
                        sprint.name,
                        sprint.issues.len()
                    );
                } else {
                    warn!("No active sprint found or board ID not configured");
                }
            }
            Err(e) => {
                error!("Failed to fetch current sprint: {}", e);
                return Err(e);
            }
        }

        match last_updated_result {
            Ok(issues) => {
                data.last_updated_issues = issues;
                info!(
                    "Successfully loaded {} recently viewed issues",
                    data.last_updated_issues.len()
                );
            }
            Err(e) => {
                error!("Failed to fetch recently viewed issues: {}", e);
                return Err(e);
            }
        }

        match board_result {
            Ok(issues) => {
                data.board_issues = issues;
                info!(
                    "Successfully loaded {} recently updated issues",
                    data.last_updated_issues.len()
                );
            }
            Err(e) => {
                error!("Failed to fetch recently updated issues: {}", e);
                return Err(e);
            }
        }

        if data.selected_issue.is_none() {
            if let Some(sprint) = &data.current_sprint {
                if let Some(first_issue) = sprint.issues.first() {
                    let first_issue_clone = first_issue.clone();
                    data.selected_issue = Some(first_issue_clone.clone());
                    debug!("Auto-selected first issue: {}", first_issue_clone.key);
                }
            }
        }

        info!("Data refresh completed successfully");
        Ok(())
    }

    async fn fetch_current_sprint(&self) -> ApiResult<Option<Sprint>> {
        if let Some(board_id) = &self.config.board_id {
            debug!("Fetching current sprint for board ID: {}", board_id);
            self.client.get_current_sprint(board_id).await
        } else {
            warn!("Board ID not configured, skipping sprint fetch. Set JIRA_BOARD_ID environment variable.");
            Ok(None)
        }
    }

    async fn fetch_board_issues(&self) -> ApiResult<Vec<JiraIssue>> {
        if let Some(board_id) = &self.config.board_id {
            debug!("Fetching current sprint for board ID: {}", board_id);
            self.client.get_board_issues(board_id).await
        } else {
            warn!("Board ID not configured, skipping sprint fetch. Set JIRA_BOARD_ID environment variable.");
            Ok(vec![])
        }
    }

    async fn fetch_last_updated_issues(&self) -> ApiResult<Vec<JiraIssue>> {
        debug!(
            "Fetching recently updated issues (max: {})",
            self.config.max_results
        );
        self.client
            .get_recently_updated_issues(self.config.max_results)
            .await
    }

    pub async fn fetch_issue_detail(&self, issue_key: &str) -> ApiResult<Option<JiraIssue>> {
        self.client.get_issue(issue_key).await
    }

    pub async fn get_issue_transitions(
        &self,
        issue_key: &str,
    ) -> ApiResult<Vec<crate::api::types::JiraTransition>> {
        self.client.get_issue_transitions(issue_key).await
    }

    pub async fn transition_issue(&self, issue_key: &str, transition_id: &str) -> ApiResult<()> {
        self.client.transition_issue(issue_key, transition_id).await
    }

    pub async fn start_auto_refresh(&self) -> ApiResult<()> {
        if let Some(interval) = self.config.auto_refresh_interval {
            let data_manager = self.clone();

            tokio::spawn(async move {
                let mut interval_timer = tokio::time::interval(interval);

                loop {
                    interval_timer.tick().await;

                    if let Err(e) = data_manager.refresh_all_data().await {
                        error!("Auto-refresh error: {}", e);
                    }
                }
            });
        }

        Ok(())
    }
}
