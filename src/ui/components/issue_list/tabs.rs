#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JqlTab {
    pub name: String,
    pub jql: String,
    pub description: Option<String>,
}

impl JqlTab {
    pub fn new(name: impl Into<String>, jql: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            jql: jql.into(),
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

pub struct TabBar {
    pub tabs: Vec<JqlTab>,
    pub selected_index: usize,
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn with_tabs(tabs: Vec<JqlTab>) -> Self {
        Self {
            tabs,
            selected_index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn add_tab(&mut self, tab: JqlTab) {
        self.tabs.push(tab);
    }

    pub fn current_tab(&self) -> Option<&JqlTab> {
        self.tabs.get(self.selected_index)
    }

    pub fn current_jql(&self) -> Option<&str> {
        self.current_tab().map(|tab| tab.jql.as_str())
    }

    pub fn move_left(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.tabs.len().saturating_sub(1);
        }
    }

    pub fn move_right(&mut self) {
        if self.selected_index + 1 < self.tabs.len() {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }

    #[allow(dead_code)]
    pub fn select_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.selected_index = index;
            true
        } else {
            false
        }
    }

    // Note: Tabs are now integrated directly into the IssueList component's title area
    // The standalone draw method is no longer needed but kept for potential future use

    /// Get the default tabs for common JQL queries
    pub fn default_tabs() -> Vec<JqlTab> {
        vec![
            JqlTab::new("All", "").with_description("All issues in current project"),
            JqlTab::new("Assigned", "assignee = currentUser()")
                .with_description("Issues assigned to you"),
            JqlTab::new("Recent", "updated >= -7d").with_description("Updated in last 7 days"),
            JqlTab::new("In Progress", "status in ('In Progress', 'In Development')")
                .with_description("Currently in progress"),
            JqlTab::new("To Do", "status = 'To Do'").with_description("Ready to start"),
            JqlTab::new("High Priority", "priority in (Highest, High)")
                .with_description("High priority issues"),
        ]
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::with_tabs(Self::default_tabs())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TabAction {
    TabChanged,
    NoAction,
}
