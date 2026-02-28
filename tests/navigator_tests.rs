use lazyjira::app::navigator::{ActiveList, Navigator};

#[test]
fn test_navigator_new() {
    let navigator = Navigator::new();
    assert_eq!(navigator.active, ActiveList::Sprint);
}

#[test]
fn test_move_right_from_sprint() {
    let mut navigator = Navigator::new();
    assert_eq!(navigator.active, ActiveList::Sprint);

    navigator.move_right();
    assert_eq!(navigator.active, ActiveList::Backlog);
}

#[test]
fn test_move_right_from_recently_updated() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::RecentlyUpdated;

    navigator.move_right();
    assert_eq!(navigator.active, ActiveList::RecentlyUpdated);
}

#[test]
fn test_move_right_from_recently_updated_stays_at_recently_updated() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::RecentlyUpdated;

    navigator.move_right();
    assert_eq!(navigator.active, ActiveList::RecentlyUpdated);
}

#[test]
fn test_move_left_from_sprint_stays_at_sprint() {
    let mut navigator = Navigator::new();
    assert_eq!(navigator.active, ActiveList::Sprint);

    navigator.move_left();
    assert_eq!(navigator.active, ActiveList::Sprint);
}

#[test]
fn test_move_left_from_recently_updated() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::RecentlyUpdated;

    navigator.move_left();
    assert_eq!(navigator.active, ActiveList::Backlog);
}

#[test]
fn test_move_left_from_backlog() {
    let mut navigator = Navigator::new();
    navigator.active = ActiveList::Backlog;

    navigator.move_left();
    assert_eq!(navigator.active, ActiveList::Sprint);
}

#[test]
fn test_navigation_cycle() {
    let mut navigator = Navigator::new();

    // Start at Sprint
    assert_eq!(navigator.active, ActiveList::Sprint);

    // Sprint -> RecentlyUpdated -> Backlog
    navigator.move_right();
    assert_eq!(navigator.active, ActiveList::Backlog);

    navigator.move_right();
    assert_eq!(navigator.active, ActiveList::RecentlyUpdated);

    // Backlog -> RecentlyUpdated -> Sprint
    navigator.move_left();
    assert_eq!(navigator.active, ActiveList::Backlog);

    navigator.move_left();
    assert_eq!(navigator.active, ActiveList::Sprint);
}

#[test]
fn test_active_list_enum_properties() {
    // Test that ActiveList implements required traits
    let sprint = ActiveList::Sprint;
    let recently_updated = ActiveList::RecentlyUpdated;
    let _backlog = ActiveList::Backlog;

    // Test PartialEq
    assert_eq!(sprint, ActiveList::Sprint);
    assert_ne!(sprint, recently_updated);

    // Test Copy/Clone
    let sprint_copy = sprint;
    assert_eq!(sprint, sprint_copy);

    // Test Debug
    let debug_str = format!("{:?}", sprint);
    assert!(debug_str.contains("Sprint"));
}
