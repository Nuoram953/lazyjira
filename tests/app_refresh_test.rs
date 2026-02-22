use chrono::Utc;
use lazyjira::models::{AppData, JiraIssue, Sprint};
use std::sync::Arc;
use tokio::sync::RwLock;

mod test_utils;

struct MockDataManager {
    data: Arc<RwLock<AppData>>,
    should_fail: bool,
    use_data_without_original: bool,
}

impl MockDataManager {
    fn new_with_data(initial_data: AppData) -> Self {
        Self {
            data: Arc::new(RwLock::new(initial_data)),
            should_fail: false,
            use_data_without_original: false,
        }
    }

    fn new_with_failure() -> Self {
        Self {
            data: Arc::new(RwLock::new(AppData::new())),
            should_fail: true,
            use_data_without_original: false,
        }
    }

    fn new_empty_with_refreshable_data() -> Self {
        Self {
            data: Arc::new(RwLock::new(AppData::new())),
            should_fail: false,
            use_data_without_original: false,
        }
    }

    fn configure_to_exclude_original(&mut self) {
        self.use_data_without_original = true;
    }

    async fn get_data(&self) -> AppData {
        self.data.read().await.clone()
    }

    async fn refresh_all_data(&self) -> Result<(), String> {
        if self.should_fail {
            return Err("Mock refresh failure".to_string());
        }

        let mut data = self.data.write().await;

        if self.use_data_without_original {
            data.selected_issue = None;
        } else {
            let new_issues = vec![
                JiraIssue {
                    key: "REFRESH-1".to_string(),
                    summary: "Refreshed issue 1".to_string(),
                    description: Some("This is a refreshed issue".to_string()),
                    status: "To Do".to_string(),
                    priority: Some("High".to_string()),
                    assignee: Some("refresh@example.com".to_string()),
                    reporter: Some("reporter@example.com".to_string()),
                    created: Utc::now(),
                    updated: Utc::now(),
                    issue_type: "Story".to_string(),
                },
                JiraIssue {
                    key: "PROJ-123".to_string(),
                    summary: "Updated user authentication".to_string(),
                    description: Some(
                        "Updated OAuth 2.0 authentication for the application".to_string(),
                    ),
                    status: "In Progress".to_string(),
                    priority: Some("High".to_string()),
                    assignee: Some("john.doe@example.com".to_string()),
                    reporter: Some("jane.smith@example.com".to_string()),
                    created: Utc::now(),
                    updated: Utc::now(),
                    issue_type: "Story".to_string(),
                },
            ];

            data.current_sprint = Some(Sprint {
                id: "REFRESH-SPRINT-1".to_string(),
                name: "Refreshed Sprint 1".to_string(),
                state: "Active".to_string(),
                start_date: Some(Utc::now()),
                end_date: Some(Utc::now()),
                issues: new_issues.clone(),
            });

            data.board_issues = new_issues.clone();
            data.last_updated_issues = new_issues;

            data.selected_issue = None;
        }

        Ok(())
    }

    async fn set_new_data_without_original_issue(&self) {
        let mut data = self.data.write().await;

        let new_issues = vec![
            JiraIssue {
                key: "NEW-1".to_string(),
                summary: "Brand new issue 1".to_string(),
                description: Some("This is a brand new issue".to_string()),
                status: "To Do".to_string(),
                priority: Some("Medium".to_string()),
                assignee: Some("new@example.com".to_string()),
                reporter: Some("reporter@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Task".to_string(),
            },
            JiraIssue {
                key: "NEW-2".to_string(),
                summary: "Brand new issue 2".to_string(),
                description: Some("This is another brand new issue".to_string()),
                status: "Done".to_string(),
                priority: Some("Low".to_string()),
                assignee: Some("another@example.com".to_string()),
                reporter: Some("reporter@example.com".to_string()),
                created: Utc::now(),
                updated: Utc::now(),
                issue_type: "Bug".to_string(),
            },
        ];

        data.current_sprint = Some(Sprint {
            id: "NEW-SPRINT".to_string(),
            name: "New Sprint".to_string(),
            state: "Active".to_string(),
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now()),
            issues: new_issues.clone(),
        });

        data.board_issues = new_issues.clone();
        data.last_updated_issues = new_issues;

        data.selected_issue = None;
    }
}

struct TestApp {
    pub data: AppData,
    pub data_manager: Option<MockDataManager>,
}

impl TestApp {
    fn new_with_mock_data() -> Self {
        let mock_data = AppData::with_mock_data();
        let data_manager = MockDataManager::new_with_data(mock_data.clone());

        Self {
            data: mock_data,
            data_manager: Some(data_manager),
        }
    }

    fn new_with_failure_manager() -> Self {
        let mock_data = AppData::with_mock_data();
        let data_manager = MockDataManager::new_with_failure();

        Self {
            data: mock_data,
            data_manager: Some(data_manager),
        }
    }

    async fn refresh_data(&mut self) {
        if let Some(data_manager) = &self.data_manager {
            let selected_issue_key = self
                .data
                .selected_issue
                .as_ref()
                .map(|issue| issue.key.clone());

            match data_manager.refresh_all_data().await {
                Ok(_) => {
                    self.data = data_manager.get_data().await;

                    if let Some(key) = selected_issue_key {
                        self.restore_selected_issue_by_key(&key);
                    }
                }
                Err(e) => {
                    log::error!("Error refreshing data: {}", e);
                }
            }
        }
    }

    fn restore_selected_issue_by_key(&mut self, key: &str) {
        if let Some(sprint) = &self.data.current_sprint {
            if let Some(issue) = sprint.issues.iter().find(|issue| issue.key == key) {
                self.data.selected_issue = Some(issue.clone());
                return;
            }
        }

        if let Some(issue) = self.data.board_issues.iter().find(|issue| issue.key == key) {
            self.data.selected_issue = Some(issue.clone());
            return;
        }

        if let Some(issue) = self
            .data
            .last_updated_issues
            .iter()
            .find(|issue| issue.key == key)
        {
            self.data.selected_issue = Some(issue.clone());
            return;
        }

        self.data.selected_issue = None;
    }
}

#[tokio::test]
async fn test_refresh_preserves_selected_issue() {
    let mut app = TestApp::new_with_mock_data();

    assert!(app.data.selected_issue.is_some());
    let original_key = app.data.selected_issue.as_ref().unwrap().key.clone();
    assert_eq!(original_key, "PROJ-123");

    app.refresh_data().await;

    assert!(app.data.selected_issue.is_some());
    assert_eq!(app.data.selected_issue.as_ref().unwrap().key, original_key);

    assert_eq!(
        app.data.selected_issue.as_ref().unwrap().summary,
        "Updated user authentication"
    );
}

#[tokio::test]
async fn test_refresh_handles_missing_issue_gracefully() {
    let mut app = TestApp::new_with_mock_data();

    assert!(app.data.selected_issue.is_some());
    let original_key = app.data.selected_issue.as_ref().unwrap().key.clone();
    assert_eq!(original_key, "PROJ-123");

    if let Some(data_manager) = &mut app.data_manager {
        data_manager.set_new_data_without_original_issue().await;
        data_manager.configure_to_exclude_original();
    }

    app.refresh_data().await;

    assert!(app.data.selected_issue.is_none());

    assert!(app.data.current_sprint.is_some());
    let sprint = app.data.current_sprint.as_ref().unwrap();
    assert_eq!(sprint.name, "New Sprint");
    assert!(!sprint.issues.is_empty());

    let has_original_issue = sprint.issues.iter().any(|issue| issue.key == original_key);
    assert!(!has_original_issue);
}

#[tokio::test]
async fn test_refresh_with_no_initial_selection() {
    let mut app = TestApp::new_with_mock_data();

    app.data.selected_issue = None;

    app.refresh_data().await;

    assert!(app.data.selected_issue.is_none());

    assert!(app.data.current_sprint.is_some());
    assert!(!app.data.board_issues.is_empty());
    assert!(!app.data.last_updated_issues.is_empty());
}

#[tokio::test]
async fn test_refresh_with_empty_initial_data() {
    let mut app = TestApp {
        data: AppData::new(),
        data_manager: Some(MockDataManager::new_empty_with_refreshable_data()),
    };

    assert!(app.data.selected_issue.is_none());
    assert!(app.data.current_sprint.is_none());
    assert!(app.data.board_issues.is_empty());

    app.refresh_data().await;

    assert!(app.data.selected_issue.is_none());
    assert!(app.data.current_sprint.is_some());
    assert!(!app.data.board_issues.is_empty());
}

#[tokio::test]
async fn test_refresh_failure_preserves_existing_data() {
    let mut app = TestApp::new_with_failure_manager();

    let original_selected_key = app.data.selected_issue.as_ref().map(|i| i.key.clone());
    let original_sprint_name = app.data.current_sprint.as_ref().map(|s| s.name.clone());

    app.refresh_data().await;

    assert_eq!(
        app.data.selected_issue.as_ref().map(|i| i.key.clone()),
        original_selected_key
    );
    assert_eq!(
        app.data.current_sprint.as_ref().map(|s| s.name.clone()),
        original_sprint_name
    );
}

#[tokio::test]
async fn test_restore_selected_issue_finds_issue_in_different_lists() {
    let mut app = TestApp::new_with_mock_data();

    let test_issue = JiraIssue {
        key: "BOARD-ONLY-1".to_string(),
        summary: "Issue only in board".to_string(),
        description: Some("This issue exists only in board issues".to_string()),
        status: "To Do".to_string(),
        priority: Some("Medium".to_string()),
        assignee: Some("board@example.com".to_string()),
        reporter: Some("reporter@example.com".to_string()),
        created: Utc::now(),
        updated: Utc::now(),
        issue_type: "Story".to_string(),
    };

    app.data.selected_issue = Some(test_issue.clone());

    if let Some(data_manager) = &mut app.data_manager {
        let mut data = data_manager.data.write().await;

        let regular_issues = vec![JiraIssue {
            key: "REGULAR-1".to_string(),
            summary: "Regular issue 1".to_string(),
            description: Some("This is a regular issue".to_string()),
            status: "To Do".to_string(),
            priority: Some("Medium".to_string()),
            assignee: Some("regular@example.com".to_string()),
            reporter: Some("reporter@example.com".to_string()),
            created: Utc::now(),
            updated: Utc::now(),
            issue_type: "Story".to_string(),
        }];

        data.current_sprint = Some(Sprint {
            id: "TEST-SPRINT".to_string(),
            name: "Test Sprint".to_string(),
            state: "Active".to_string(),
            start_date: Some(Utc::now()),
            end_date: Some(Utc::now()),
            issues: regular_issues.clone(),
        });
        data.last_updated_issues = regular_issues;

        let mut board_issues = vec![test_issue.clone()];
        board_issues.extend(vec![JiraIssue {
            key: "BOARD-REG-1".to_string(),
            summary: "Board regular issue 1".to_string(),
            description: Some("This is a regular board issue".to_string()),
            status: "To Do".to_string(),
            priority: Some("Low".to_string()),
            assignee: Some("board-regular@example.com".to_string()),
            reporter: Some("reporter@example.com".to_string()),
            created: Utc::now(),
            updated: Utc::now(),
            issue_type: "Task".to_string(),
        }]);
        data.board_issues = board_issues;

        data_manager.use_data_without_original = true;
    }

    app.refresh_data().await;

    assert!(app.data.selected_issue.is_some());
    assert_eq!(
        app.data.selected_issue.as_ref().unwrap().key,
        "BOARD-ONLY-1"
    );
}
