#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    Main,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusedPane {
    Sprint,
    LastUpdated,
    Board,
    Detail,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Quit,
    ShowHelp,
    Navigate(Direction),
    SelectItem,
    GoBack,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct NavigationState {
    pub current_view: AppView,
    pub focused_pane: FocusedPane,
    pub previous_pane: Option<FocusedPane>,
    pub sprint_selected: usize,
    pub last_viewed_selected: usize,
    pub last_updated_selected: usize,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

impl NavigationState {
    pub fn new() -> Self {
        Self {
            current_view: AppView::Main,
            focused_pane: FocusedPane::Sprint,
            previous_pane: None,
            sprint_selected: 0,
            last_viewed_selected: 0,
            last_updated_selected: 0,
        }
    }

    pub fn move_focus(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => match self.focused_pane {
                FocusedPane::Sprint => self.focused_pane = FocusedPane::Sprint,
                FocusedPane::LastUpdated => self.focused_pane = FocusedPane::Sprint,
                FocusedPane::Board => self.focused_pane = FocusedPane::LastUpdated,
                FocusedPane::Detail => {}
            },
            Direction::Right => match self.focused_pane {
                FocusedPane::Sprint => self.focused_pane = FocusedPane::LastUpdated,
                FocusedPane::LastUpdated => self.focused_pane = FocusedPane::Board,
                FocusedPane::Board => self.focused_pane = FocusedPane::Board,
                FocusedPane::Detail => {}
            },
        }
    }

    pub fn move_selection(&mut self, direction: Direction, max_items: usize) {
        match self.focused_pane {
            FocusedPane::Sprint => match direction {
                Direction::Down => {
                    if self.sprint_selected + 1 < max_items {
                        self.sprint_selected = self.sprint_selected.saturating_add(1);
                    }
                }
                Direction::Up => self.sprint_selected = self.sprint_selected.saturating_sub(1),
                Direction::Left | Direction::Right => {}
            },
            FocusedPane::Board => match direction {
                Direction::Down => {
                    if self.last_viewed_selected + 1 < max_items {
                        self.last_viewed_selected = self.last_viewed_selected.saturating_add(1);
                    }
                }
                Direction::Up => {
                    self.last_viewed_selected = self.last_viewed_selected.saturating_sub(1)
                }
                Direction::Left | Direction::Right => {}
            },
            FocusedPane::LastUpdated => match direction {
                Direction::Down => {
                    if self.last_updated_selected + 1 < max_items {
                        self.last_updated_selected = self.last_updated_selected.saturating_add(1);
                    }
                }
                Direction::Up => {
                    self.last_updated_selected = self.last_updated_selected.saturating_sub(1)
                }
                Direction::Left | Direction::Right => {}
            },
            FocusedPane::Detail => {}
        }
    }

    pub fn focus_detail(&mut self) {
        if matches!(
            self.focused_pane,
            FocusedPane::Sprint | FocusedPane::Board | FocusedPane::LastUpdated
        ) {
            self.previous_pane = Some(self.focused_pane.clone());
            self.focused_pane = FocusedPane::Detail;
        }
    }

    pub fn go_back_from_detail(&mut self) {
        if self.focused_pane == FocusedPane::Detail {
            self.focused_pane = self.previous_pane.clone().unwrap_or(FocusedPane::Sprint);
        }
    }
}
