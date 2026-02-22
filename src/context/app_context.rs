use crate::api::types::JiraTransition;
use crate::data_manager::DataManager;
use crate::models::AppData;
use crate::navigation::NavigationState;

#[derive(Debug)]
pub struct AppContext {
    pub should_quit: bool,
    pub data: AppData,
    pub navigation: NavigationState,
    pub data_manager: Option<DataManager>,
    pub use_api: bool,
    pub available_transitions: Vec<JiraTransition>,
    pub transitioning_issue_key: Option<String>,
}

impl AppContext {
    pub fn new(
        data: AppData,
        navigation: NavigationState,
        data_manager: Option<DataManager>,
        use_api: bool,
    ) -> Self {
        Self {
            should_quit: false,
            data,
            navigation,
            data_manager,
            use_api,
            available_transitions: Vec::new(),
            transitioning_issue_key: None,
        }
    }

    pub fn get_selected_issue_key(&self) -> Option<String> {
        use crate::navigation::FocusedPane;

        match self.navigation.focused_pane {
            FocusedPane::Sprint => {
                if let Some(sprint) = &self.data.current_sprint {
                    sprint
                        .issues
                        .get(self.navigation.sprint_selected)
                        .map(|issue| issue.key.clone())
                } else {
                    None
                }
            }
            FocusedPane::Board => self
                .data
                .board_issues
                .get(self.navigation.last_viewed_selected)
                .map(|issue| issue.key.clone()),
            FocusedPane::LastUpdated => self
                .data
                .last_updated_issues
                .get(self.navigation.last_updated_selected)
                .map(|issue| issue.key.clone()),
            FocusedPane::Detail => self
                .data
                .selected_issue
                .as_ref()
                .map(|issue| issue.key.clone()),
            _ => None,
        }
    }

    pub async fn refresh_data(&mut self) {
        if let Some(data_manager) = &self.data_manager {
            let new_data = data_manager.get_data().await;
            let selected_issue_key = self.data.selected_issue.as_ref().map(|i| i.key.clone());
            self.data = new_data;

            if let Some(key) = selected_issue_key {
                self.restore_selected_issue_by_key(&key);
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
