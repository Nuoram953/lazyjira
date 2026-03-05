use super::{App, AppAction, AppMessage};
use crate::app::keybinds::{DetailAction, GlobalAction};
use crate::app::ActiveList;
use crate::ui::components::{ListAction, TabAction};
use crate::{
    app::keybinds::{DETAIL_KEYBINDS, GLOBAL_KEYBINDS},
    core::event::AppEvent,
};
use crossterm::event::KeyCode;

impl App {
    pub fn handle_event(&mut self, event: AppEvent) -> Option<AppAction> {
        match event {
            AppEvent::Key(key) => {
                if self.search_mode {
                    match key.code {
                        KeyCode::Char(c) => self.search_query.push(c),
                        KeyCode::Backspace => {
                            self.search_query.pop();
                        }
                        _ => {}
                    }
                    return None;
                }

                if self.detail_view.focused {
                    if let Some(bind) = DETAIL_KEYBINDS.iter().find(|b| b.key == key.code) {
                        match bind.action {
                            DetailAction::Up => {
                                self.detail_view.move_up();
                            }
                            DetailAction::Down => {
                                self.detail_view.move_down();
                            }
                            DetailAction::Edit => {
                                if !self.detail_view.edit_mode {
                                    self.detail_view.enter_edit_mode();
                                }
                            }
                            DetailAction::Esc => {
                                if self.detail_view.edit_mode {
                                    self.detail_view.exit_edit_mode();
                                } else {
                                    self.detail_view.focused = false;
                                }
                            }
                        }
                        return None;
                    }
                }

                if let Some(bind) = GLOBAL_KEYBINDS.iter().find(|b| b.key == key.code) {
                    match bind.action {
                        GlobalAction::Quit => return Some(AppAction::Quit),
                        GlobalAction::Search => {
                            self.search_mode = true;
                            self.search_query.clear();
                        }
                        GlobalAction::Enter => {
                            self.detail_view.focused = true;
                        }
                        GlobalAction::Esc => {
                            if !self.detail_view.focused {
                                self.search_mode = false;
                            }
                        }
                        GlobalAction::TabNext => match self.active_list_mut().move_tab_right() {
                            TabAction::TabChanged => {
                                self.fetch_tab_issues_for_active_list();
                            }
                            TabAction::NoAction => {}
                        },
                        GlobalAction::TabPrev => match self.active_list_mut().move_tab_left() {
                            TabAction::TabChanged => {
                                self.fetch_tab_issues_for_active_list();
                            }
                            TabAction::NoAction => {}
                        },
                        GlobalAction::Up => {
                            if !self.detail_view.focused {
                                match self.active_list_mut().move_up() {
                                    ListAction::RequestMore => {}
                                    ListAction::None => {}
                                    ListAction::Sort => {}
                                    ListAction::SelectionChanged => {
                                        self.fetch_detail_issue();
                                    }
                                }
                            }
                        }
                        GlobalAction::Down => {
                            if !self.detail_view.focused {
                                match self.active_list_mut().move_down() {
                                    ListAction::RequestMore => {
                                        self.fetch_more_for_active_list();
                                    }
                                    ListAction::None => {}
                                    ListAction::Sort => {}
                                    ListAction::SelectionChanged => {
                                        self.fetch_detail_issue();
                                    }
                                }
                            }
                        }
                        GlobalAction::Left => {
                            if !self.detail_view.focused {
                                self.navigator.move_left();
                                self.fetch_detail_issue();
                            }
                        }
                        GlobalAction::Right => {
                            if !self.detail_view.focused {
                                self.navigator.move_right();
                                self.fetch_detail_issue();
                            }
                        }
                        GlobalAction::CycleSort => {
                            if !self.detail_view.focused {
                                match self.active_list_mut().cycle_sort() {
                                    ListAction::RequestMore => {}
                                    ListAction::None => {}
                                    ListAction::Sort => {
                                        self.sort_for_active_list();
                                    }
                                    ListAction::SelectionChanged => {}
                                }
                            }
                        }
                    }
                }

                None
            }
            _ => None,
        }
    }

    pub fn update_from_message(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::ItemsLoaded {
                list: active,
                result,
                append,
            } => {
                let list = match active {
                    ActiveList::Sprint => &mut self.items_sprint,
                    ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
                    ActiveList::Backlog => &mut self.items_backlog,
                };

                list.is_loading = false;

                if result.items.is_empty() {
                    list.result.has_more = false;
                } else {
                    if append {
                        list.result.items.extend(result.items);
                    } else {
                        list.result = result;

                        list.result.has_more = true;
                    }

                    list.result.page += 1;

                    if !list.has_selection() {
                        list.ensure_selection();
                    }
                }
            }
            AppMessage::ItemsSorted {
                list: active,
                result,
            } => {
                let list = match active {
                    ActiveList::Sprint => &mut self.items_sprint,
                    ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
                    ActiveList::Backlog => &mut self.items_backlog,
                };

                list.result = result;
                list.is_loading = false;
                list.result.page = 1;
                list.result.has_more = true;
            }
            AppMessage::Error { list, message } => {
                let target = match list {
                    ActiveList::Sprint => &mut self.items_sprint,
                    ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
                    ActiveList::Backlog => &mut self.items_backlog,
                };

                target.is_loading = false;

                println!("Error for {:?}: {}", list, message);
            }
            AppMessage::ItemDetailLoaded { item } => {
                self.detail_view.set_issue(Some(item));
            }
        }
    }

    fn fetch_detail_issue(&mut self) {
        let active_list = self.navigator.active;
        let tx = self.tx.clone();
        let client = self.client.clone();

        if let Some(index) = self.active_list().state_selected() {
            if let Some(issue) = self.active_list().result.items.get(index) {
                let key = issue.key.clone();

                tokio::spawn(async move {
                    let result = client.fetch_issue_by_key(key).await;

                    match result {
                        Ok(item) => {
                            let _ = tx.send(AppMessage::ItemDetailLoaded { item });
                        }
                        Err(e) => {
                            let _ = tx.send(AppMessage::Error {
                                list: active_list,
                                message: e.to_string(),
                            });
                        }
                    }
                });
            }
        }
    }

    fn fetch_more_for_active_list(&mut self) {
        let active_list = self.navigator.active;
        self.fetch_issues_async(active_list, true);
    }

    fn sort_for_active_list(&mut self) {
        let active_list = self.navigator.active;
        self.fetch_issues_async(active_list, false);
    }

    fn fetch_tab_issues_for_active_list(&mut self) {
        let active_list = self.navigator.active;
        self.fetch_issues_async(active_list, false);
    }

    pub async fn fetch_issues_for_list(
        &self,
        list: ActiveList,
        sort: Option<crate::services::sort::SortMode>,
        filter: Option<String>,
        page: usize,
    ) -> Result<
        crate::services::types::Paginated<crate::services::types::JiraIssue>,
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let sort_mode = sort.unwrap_or_else(|| match list {
            ActiveList::Sprint => crate::services::sort::SortMode::PriorityDesc,
            ActiveList::RecentlyUpdated => crate::services::sort::SortMode::UpdatedDesc,
            ActiveList::Backlog => crate::services::sort::SortMode::KeyDesc,
        });

        let result = match list {
            ActiveList::Sprint => {
                self.client
                    .fetch_current_sprint_issues(sort_mode, filter, page)
                    .await
            }
            ActiveList::RecentlyUpdated => {
                self.client
                    .fetch_recently_updated_issues(sort_mode, filter, page)
                    .await
            }
            ActiveList::Backlog => {
                self.client
                    .fetch_backlog_issues(sort_mode, filter, page)
                    .await
            }
        };

        result.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    pub fn fetch_issues_async(&mut self, list: ActiveList, append: bool) {
        let active_list_state = match list {
            ActiveList::Sprint => &mut self.items_sprint,
            ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
            ActiveList::Backlog => &mut self.items_backlog,
        };

        if active_list_state.is_loading {
            return;
        }
        active_list_state.is_loading = true;

        let page = if append {
            active_list_state.result.page
        } else {
            0
        };
        let sort = active_list_state.sort_mode;
        let filter = active_list_state.current_jql();
        let tx = self.tx.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let result = match list {
                ActiveList::Sprint => client.fetch_current_sprint_issues(sort, filter, page).await,
                ActiveList::RecentlyUpdated => {
                    client
                        .fetch_recently_updated_issues(sort, filter, page)
                        .await
                }
                ActiveList::Backlog => client.fetch_backlog_issues(sort, filter, page).await,
            };

            match result {
                Ok(items) => {
                    let message = if append {
                        AppMessage::ItemsLoaded {
                            list,
                            result: items,
                            append: true,
                        }
                    } else {
                        AppMessage::ItemsSorted {
                            list,
                            result: items,
                        }
                    };
                    let _ = tx.send(message);
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error {
                        list,
                        message: e.to_string(),
                    });
                }
            }
        });
    }

    pub async fn load_initial_data(&mut self) {
        for list in [
            ActiveList::Sprint,
            ActiveList::RecentlyUpdated,
            ActiveList::Backlog,
        ] {
            let filter = match list {
                ActiveList::Sprint => self.items_sprint.current_jql(),
                ActiveList::RecentlyUpdated => self.items_recently_updated.current_jql(),
                ActiveList::Backlog => self.items_backlog.current_jql(),
            };

            if let Ok(result) = self.fetch_issues_for_list(list, None, filter, 0).await {
                let issue_list = match list {
                    ActiveList::Sprint => &mut self.items_sprint,
                    ActiveList::RecentlyUpdated => &mut self.items_recently_updated,
                    ActiveList::Backlog => &mut self.items_backlog,
                };

                issue_list.is_loading = false;
                issue_list.result = result;
                issue_list.result.page += 1;
                issue_list.result.has_more = true;

                if !issue_list.result.items.is_empty() && !issue_list.has_selection() {
                    issue_list.ensure_selection();
                }

                self.fetch_detail_issue();
            }
        }
    }
}
