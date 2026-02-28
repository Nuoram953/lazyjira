use super::{App, AppAction, AppMessage};
use crate::app::keybinds::ActionKey;
use crate::app::ActiveList;
use crate::ui::components::{ListAction, TabAction};
use crate::{app::keybinds::GLOBAL_KEYBINDS, core::event::AppEvent};
use crossterm::event::KeyCode;

impl App {
    pub fn handle_event(&mut self, event: AppEvent) -> Option<AppAction> {
        match event {
            AppEvent::Key(key) => {
                if let Some(bind) = GLOBAL_KEYBINDS.iter().find(|b| b.key == key.code) {
                    match bind.action {
                        ActionKey::Quit => return Some(AppAction::Quit),
                        ActionKey::Search => {
                            self.search_mode = true;
                            self.search_query.clear();
                        }
                        ActionKey::Esc => {
                            self.search_mode = false;
                        }
                        ActionKey::TabNext => match self.active_list_mut().move_tab_right() {
                            TabAction::TabChanged => {
                                self.fetch_tab_issues_for_active_list();
                            }
                            TabAction::NoAction => {}
                        },
                        ActionKey::TabPrev => match self.active_list_mut().move_tab_left() {
                            TabAction::TabChanged => {
                                self.fetch_tab_issues_for_active_list();
                            }
                            TabAction::NoAction => {}
                        },
                        ActionKey::Up => self.active_list_mut().move_up(),
                        ActionKey::Down => match self.active_list_mut().move_down() {
                            ListAction::RequestMore => {
                                self.fetch_more_for_active_list();
                            }
                            ListAction::None => {}
                            ListAction::Sort => {}
                        },
                        ActionKey::Left => self.navigator.move_left(),
                        ActionKey::Right => self.navigator.move_right(),
                        ActionKey::CycleSort => match self.active_list_mut().cycle_sort() {
                            ListAction::RequestMore => {}
                            ListAction::None => {}
                            ListAction::Sort => {
                                self.sort_for_active_list();
                            }
                        },
                    }
                } else if self.search_mode {
                    match key.code {
                        KeyCode::Char(c) => self.search_query.push(c),
                        KeyCode::Backspace => {
                            self.search_query.pop();
                        }
                        _ => {}
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
        }
    }

    fn fetch_more_for_active_list(&mut self) {
        let active_list = self.navigator.active;

        let active_list_state = self.active_list_mut();
        if active_list_state.is_loading {
            return;
        }
        active_list_state.is_loading = true;

        let page = self.active_list().result.page;
        let sort = self.active_list().sort_mode;
        let tx = self.tx.clone();
        let client = self.client.clone();

        let filter = self.active_list().current_jql();

        tokio::spawn(async move {
            let result = match active_list {
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
                    let _ = tx.send(AppMessage::ItemsLoaded {
                        list: active_list,
                        result: items,
                        append: true,
                    });
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

    fn sort_for_active_list(&mut self) {
        let active_list = self.navigator.active;

        let active_list_state = self.active_list_mut();
        if active_list_state.is_loading {
            return;
        }
        active_list_state.is_loading = true;

        let page = 0;
        let sort = self.active_list().sort_mode;
        let tx = self.tx.clone();
        let client = self.client.clone();

        let filter = self.active_list().current_jql();

        tokio::spawn(async move {
            let result = match active_list {
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
                    let _ = tx.send(AppMessage::ItemsSorted {
                        list: active_list,
                        result: items,
                    });
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

    fn fetch_tab_issues_for_active_list(&mut self) {
        let active_list = self.navigator.active;

        let active_list_state = self.active_list_mut();
        if active_list_state.is_loading {
            return;
        }
        active_list_state.is_loading = true;

        let page = 0;
        let sort = self.active_list().sort_mode;
        let tx = self.tx.clone();
        let client = self.client.clone();

        let filter = self.active_list().current_jql();

        tokio::spawn(async move {
            let result = match active_list {
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
                    let _ = tx.send(AppMessage::ItemsSorted {
                        list: active_list,
                        result: items,
                    });
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
