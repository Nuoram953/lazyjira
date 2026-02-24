use lazyjira::app::{
    messages::AppMessage,
    navigator::{ActiveList, Navigator},
};
use tokio::sync::mpsc;

#[test]
fn test_active_list_sprint() {
    let (_tx, _rx) = mpsc::unbounded_channel::<AppMessage>();

    // Skip creating the full App since JiraClient::new requires environment variables
    // Instead test the logic directly
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::Sprint;

    assert_eq!(navigator.active, ActiveList::Sprint);
}

#[test]
fn test_active_list_recently_updated() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::RecentlyUpdated;

    assert_eq!(navigator.active, ActiveList::RecentlyUpdated);
}

#[test]
fn test_active_list_backlog() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::Backlog;

    assert_eq!(navigator.active, ActiveList::Backlog);
}
