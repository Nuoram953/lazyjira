use chrono::Utc;
use lazyjira::models::{JiraIssue, Sprint};
use std::collections::HashMap;

pub struct MockJiraClient {
    pub mock_issues: HashMap<String, JiraIssue>,
    pub mock_sprints: HashMap<String, Sprint>,
    pub should_fail: bool,
}

impl MockJiraClient {
    pub fn new() -> Self {
        let mut mock_issues = HashMap::new();
        let mut mock_sprints = HashMap::new();

        let test_issues = vec![
            JiraIssue {
                key: "MOCK-1".to_string(),
                summary: "Test issue 1".to_string(),
                description: Some("Description for test issue 1".to_string()),
                status: "To Do".to_string(),
                priority: Some("High".to_string()),
                assignee: Some("test1@example.com".to_string()),
                reporter: Some("reporter@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Story".to_string(),
            },
            JiraIssue {
                key: "MOCK-2".to_string(),
                summary: "Test issue 2".to_string(),
                description: Some("Description for test issue 2".to_string()),
                status: "In Progress".to_string(),
                priority: Some("Medium".to_string()),
                assignee: Some("test2@example.com".to_string()),
                reporter: Some("reporter@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Bug".to_string(),
            },
        ];

        for issue in &test_issues {
            mock_issues.insert(issue.key.clone(), issue.clone());
        }

        let test_sprint = Sprint {
            id: "MOCK-SPRINT-1".to_string(),
            name: "Mock Sprint 1".to_string(),
            state: "Active".to_string(),
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now()),
            issues: test_issues,
        };

        mock_sprints.insert(test_sprint.id.clone(), test_sprint);

        Self {
            mock_issues,
            mock_sprints,
            should_fail: false,
        }
    }

    pub fn set_should_fail(&mut self, should_fail: bool) {
        self.should_fail = should_fail;
    }

    pub async fn get_issues_by_board_id(&self, _board_id: &str) -> Result<Vec<JiraIssue>, String> {
        if self.should_fail {
            return Err("Mock API failure".to_string());
        }
        Ok(self.mock_issues.values().cloned().collect())
    }

    pub async fn get_current_sprint(&self, _board_id: &str) -> Result<Option<Sprint>, String> {
        if self.should_fail {
            return Err("Mock API failure".to_string());
        }
        Ok(self.mock_sprints.values().next().cloned())
    }

    pub async fn get_recently_updated_issues(
        &self,
        _max_results: usize,
    ) -> Result<Vec<JiraIssue>, String> {
        if self.should_fail {
            return Err("Mock API failure".to_string());
        }
        Ok(self.mock_issues.values().take(2).cloned().collect())
    }

    pub async fn get_issue(&self, issue_key: &str) -> Result<Option<JiraIssue>, String> {
        if self.should_fail {
            return Err("Mock API failure".to_string());
        }
        Ok(self.mock_issues.get(issue_key).cloned())
    }

    pub fn add_mock_issue(&mut self, issue: JiraIssue) {
        self.mock_issues.insert(issue.key.clone(), issue);
    }

    pub fn add_mock_sprint(&mut self, sprint: Sprint) {
        self.mock_sprints.insert(sprint.id.clone(), sprint);
    }
}

impl Default for MockJiraClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_client_basic_operations() {
        let client = MockJiraClient::new();

        let issues = client.get_issues_by_board_id("test-board").await.unwrap();
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().any(|i| i.key == "MOCK-1"));

        let sprint = client.get_current_sprint("test-board").await.unwrap();
        assert!(sprint.is_some());
        let sprint = sprint.unwrap();
        assert_eq!(sprint.name, "Mock Sprint 1");
        assert_eq!(sprint.issues.len(), 2);

        let recent_issues = client.get_recently_updated_issues(10).await.unwrap();
        assert!(!recent_issues.is_empty());

        let issue = client.get_issue("MOCK-1").await.unwrap();
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().summary, "Test issue 1");

        let missing_issue = client.get_issue("NONEXISTENT").await.unwrap();
        assert!(missing_issue.is_none());
    }

    #[tokio::test]
    async fn test_mock_client_failure_simulation() {
        let mut client = MockJiraClient::new();
        client.set_should_fail(true);

        assert!(client.get_issues_by_board_id("test-board").await.is_err());
        assert!(client.get_current_sprint("test-board").await.is_err());
        assert!(client.get_recently_updated_issues(10).await.is_err());
        assert!(client.get_issue("MOCK-1").await.is_err());
    }

    #[tokio::test]
    async fn test_mock_client_custom_data() {
        let mut client = MockJiraClient::new();

        let custom_issue = JiraIssue {
            key: "CUSTOM-1".to_string(),
            summary: "Custom test issue".to_string(),
            description: None,
            status: "Done".to_string(),
            priority: Some("Low".to_string()),
            assignee: None,
            reporter: None,
            created: Utc::now(),
            updated: Utc::now(),
            issue_type: "Task".to_string(),
        };

        client.add_mock_issue(custom_issue.clone());

        let issue = client.get_issue("CUSTOM-1").await.unwrap();
        assert!(issue.is_some());
        assert_eq!(issue.unwrap().summary, "Custom test issue");

        let issues = client.get_issues_by_board_id("test").await.unwrap();
        assert!(issues.iter().any(|i| i.key == "CUSTOM-1"));
    }

    #[tokio::test]
    async fn test_mock_client_sprint_operations() {
        let mut client = MockJiraClient::new();

        let custom_sprint = Sprint {
            id: "CUSTOM-SPRINT".to_string(),
            name: "Custom Sprint".to_string(),
            state: "Active".to_string(),
            start_date: Some(Utc::now()),
            end_date: None,
            issues: vec![],
        };

        client.add_mock_sprint(custom_sprint.clone());

        assert!(client.mock_sprints.contains_key("CUSTOM-SPRINT"));
    }
}
