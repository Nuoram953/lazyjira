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

    pub fn issue_detail(&self, issue_key: &str) -> String {
        format!("{}/rest/api/2/issue/{}", self.base_url, issue_key)
    }

    pub fn active_sprint(&self, board_id: &str) -> String {
        format!(
            "{}/rest/agile/1.0/board/{}/sprint?state=active",
            self.base_url, board_id
        )
    }

    pub fn board_issues(&self, board_id: &str) -> String {
        format!("{}/rest/agile/1.0/board/{}/issue", self.base_url, board_id)
    }

    pub fn sprint_issues(&self, sprint_id: &str) -> String {
        format!(
            "{}/rest/agile/1.0/sprint/{}/issue",
            self.base_url, sprint_id
        )
    }

    pub fn issue_transitions(&self, issue_key: &str) -> String {
        format!(
            "{}/rest/api/2/issue/{}/transitions",
            self.base_url, issue_key
        )
    }

    pub fn transition_issue(&self, issue_key: &str) -> String {
        format!(
            "{}/rest/api/2/issue/{}/transitions",
            self.base_url, issue_key
        )
    }

    #[allow(dead_code)]
    pub fn board_list(&self) -> String {
        format!("{}/rest/agile/1.0/board", self.base_url)
    }

    #[allow(dead_code)]
    pub fn user_activity(&self) -> String {
        format!("{}/rest/api/2/user/search/activity", self.base_url)
    }

    #[allow(dead_code)]
    pub fn project_list(&self) -> String {
        format!("{}/rest/api/2/project", self.base_url)
    }
}
