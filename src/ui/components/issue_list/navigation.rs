use ratatui::widgets::ListState;

#[derive(Debug, PartialEq, Eq)]
pub enum ListAction {
    None,
    RequestMore,
    SelectionChanged,
    Sort,
}

pub struct ListNavigator {
    state: ListState,
}

impl ListNavigator {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(None);
        Self { state }
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn move_up(&mut self, item_count: usize) -> ListAction {
        if item_count == 0 {
            return ListAction::None;
        }

        if let Some(selected) = self.state.selected() {
            if selected > 0 {
                self.state.select(Some(selected - 1));
                return ListAction::SelectionChanged;
            }
        } else {
            self.state.select(Some(0));
        }

        ListAction::None
    }

    pub fn move_down(&mut self, item_count: usize, has_more: bool, is_loading: bool) -> ListAction {
        if item_count == 0 {
            return ListAction::None;
        }

        if let Some(selected) = self.state.selected() {
            if selected < item_count.saturating_sub(1) {
                self.state.select(Some(selected + 1));
                return ListAction::SelectionChanged;
            }

            if has_more && !is_loading {
                return ListAction::RequestMore;
            }
        } else {
            self.state.select(Some(0));
        }

        ListAction::None
    }

    // pub fn select_first_if_available(&mut self, item_count: usize) {
    //     if item_count > 0 {
    //         self.state.select(Some(0));
    //     }
    // }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }
}

impl Default for ListNavigator {
    fn default() -> Self {
        Self::new()
    }
}
