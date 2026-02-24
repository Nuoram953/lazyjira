#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    KeyAsc,
    KeyDesc,
    UpdatedAsc,
    UpdatedDesc,
    PriorityAsc,
    PriorityDesc,
}

impl SortMode {
    pub fn jql_order_by(&self) -> &'static str {
        match self {
            SortMode::KeyAsc => "ORDER BY key ASC",
            SortMode::KeyDesc => "ORDER BY key DESC",
            SortMode::UpdatedAsc => "ORDER BY updated ASC",
            SortMode::UpdatedDesc => "ORDER BY updated DESC",
            SortMode::PriorityAsc => "ORDER BY priority ASC",
            SortMode::PriorityDesc => "ORDER BY priority DESC",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SortMode::KeyAsc => "Key ↑",
            SortMode::KeyDesc => "Key ↓",
            SortMode::UpdatedAsc => "Updated ↑",
            SortMode::UpdatedDesc => "Updated ↓",
            SortMode::PriorityAsc => "Priority ↓",
            SortMode::PriorityDesc => "Priority ↑",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SortMode::KeyAsc => SortMode::KeyDesc,
            SortMode::KeyDesc => SortMode::UpdatedAsc,
            SortMode::UpdatedAsc => SortMode::UpdatedDesc,
            SortMode::UpdatedDesc => SortMode::PriorityAsc,
            SortMode::PriorityAsc => SortMode::PriorityDesc,
            SortMode::PriorityDesc => SortMode::KeyAsc,
        }
    }
}
