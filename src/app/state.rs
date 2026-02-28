use crate::{
    app::{navigator::Navigator, ActiveList},
    services::{sort::SortMode, JiraClient},
    ui::components::{IssueList, JqlTab},
};

use super::AppMessage;
use ratatui::widgets::ListState;
use tokio::sync::mpsc::UnboundedSender;

pub struct App {
    pub items_sprint: IssueList,
    pub items_recently_updated: IssueList,
    pub items_backlog: IssueList,
    pub navigator: Navigator,
    pub client: JiraClient,
    pub search_mode: bool,
    pub search_query: String,
    pub loading: bool,
    pub should_quit: bool,
    pub tx: UnboundedSender<AppMessage>,
}

impl App {
    pub fn new(tx: UnboundedSender<AppMessage>) -> Self {
        let mut selected_top = ListState::default();
        selected_top.select(Some(0));

        let client = JiraClient::new().expect("Failed to ");

        Self {
            items_sprint: IssueList::new(
                "Sprint Issues".to_string(),
                false,
                SortMode::PriorityDesc,
            )
            .with_tabs(vec![
                JqlTab::new("My Issues", "assignee = currentUser()")
                    .with_description("Issues assigned to you in this sprint"),
                JqlTab::new("In Progress", "status=\"In Progress\"")
                    .with_description("Sprint issues currently being worked on"),
                JqlTab::new("Unassigned", "assignee is EMPTY")
                    .with_description("Sprint issues currently being worked on"),
                JqlTab::new("All", "").with_description("Sprint issues currently being worked on"),
            ]),
            items_backlog: IssueList::new("Backlog".to_string(), false, SortMode::KeyDesc),
            items_recently_updated: IssueList::new(
                "Recently updated".to_string(),
                true,
                SortMode::UpdatedDesc,
            ),
            navigator: Navigator::new(),
            client,
            search_mode: false,
            search_query: String::new(),
            loading: false,
            should_quit: false,
            tx,
        }
    }

    pub fn active_list_mut(&mut self) -> &mut IssueList {
        match self.navigator.active {
            ActiveList::Sprint => &mut self.items_sprint,
            ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
            ActiveList::Backlog => &mut self.items_backlog,
        }
    }

    pub fn active_list(&self) -> &IssueList {
        match self.navigator.active {
            ActiveList::Sprint => &self.items_sprint,
            ActiveList::RecentlyUpdated => &self.items_recently_updated,
            ActiveList::Backlog => &self.items_backlog,
        }
    }
}
