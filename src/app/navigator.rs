#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveList {
    Sprint,
    RecentlyUpdated,
    Backlog,
}

pub struct Navigator {
    pub active: ActiveList,
}

impl Default for Navigator {
    fn default() -> Self {
        Self::new()
    }
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            active: ActiveList::Sprint,
        }
    }

    pub fn move_left(&mut self) {
        self.active = match self.active {
            ActiveList::Sprint => ActiveList::Sprint,
            ActiveList::RecentlyUpdated => ActiveList::Sprint,
            ActiveList::Backlog => ActiveList::RecentlyUpdated,
        };
    }

    pub fn move_right(&mut self) {
        self.active = match self.active {
            ActiveList::Sprint => ActiveList::RecentlyUpdated,
            ActiveList::RecentlyUpdated => ActiveList::Backlog,
            ActiveList::Backlog => ActiveList::Backlog,
        };
    }
}
