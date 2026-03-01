#[derive(Debug, Clone)]
pub struct JiraEndpoints {
    base_url: String,
}

impl JiraEndpoints {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn search_issues(&self) -> String {
        format!("{}/rest/api/3/search/jql", self.base_url)
    }

    pub fn active_sprint(&self, board_id: &str) -> String {
        format!(
            "{}/rest/agile/1.0/board/{}/sprint?state=active",
            self.base_url, board_id
        )
    }

    pub fn backlog_issues(&self, board_id: &str) -> String {
        format!(
            "{}/rest/agile/1.0/board/{}/backlog",
            self.base_url, board_id
        )
    }

    pub fn get_issue(&self, key: &str) -> String {
        format!("{}/rest/api/3/issue/{}", self.base_url, key)
    }

    pub fn get_all_issues_for_sprint(&self, board_id: &str, sprint_id: &str) -> String {
        format!(
            "{}/rest/agile/1.0/board/{}/sprint/{}/issue",
            self.base_url, board_id, sprint_id
        )
    }
}
