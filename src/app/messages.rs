use crate::{
    app::ActiveList,
    services::types::{JiraIssue, Paginated},
};

pub enum AppMessage {
    ItemsLoaded {
        list: ActiveList,
        result: Paginated<JiraIssue>,
        append: bool,
    },
    ItemDetailLoaded {
        item: JiraIssue,
    },
    ItemsSorted {
        list: ActiveList,
        result: Paginated<JiraIssue>,
    },
    Error {
        list: ActiveList,
        message: String,
    },
}
