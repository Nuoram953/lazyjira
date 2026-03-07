use crossterm::event::KeyCode;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalAction {
    Quit,
    Search,
    Help,
    Esc,
    TabNext,
    TabPrev,
    Up,
    Down,
    Left,
    Right,
    CycleSort,
    Enter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetailAction {
    Up,
    Down,
    Edit,
    Esc,
}

pub struct GlobalKeyBind {
    pub key: KeyCode,
    pub action: GlobalAction,
    pub short_description: &'static str,
    #[allow(dead_code)]
    pub description: &'static str,
}

pub struct DetailKeyBind {
    pub key: KeyCode,
    pub action: DetailAction,
    pub short_description: &'static str,
    #[allow(dead_code)]
    pub description: &'static str,
}

pub static GLOBAL_KEYBINDS: Lazy<Vec<GlobalKeyBind>> = Lazy::new(|| {
    vec![
        GlobalKeyBind {
            key: KeyCode::Enter,
            action: GlobalAction::Enter,
            short_description: "Focus detail",
            description: "Switch focus to the detail pane for editing",
        },
        GlobalKeyBind {
            key: KeyCode::Char('?'),
            action: GlobalAction::Help,
            short_description: "Help",
            description: "Open this help menu with keybind reference",
        },
        GlobalKeyBind {
            key: KeyCode::Char('q'),
            action: GlobalAction::Quit,
            short_description: "Quit",
            description: "Exit the application completely",
        },
        GlobalKeyBind {
            key: KeyCode::Char('/'),
            action: GlobalAction::Search,
            short_description: "Search",
            description: "Enter search mode to filter issues",
        },
        GlobalKeyBind {
            key: KeyCode::Esc,
            action: GlobalAction::Esc,
            short_description: "Cancel/Exit",
            description: "Cancel current operation or exit search mode",
        },
        GlobalKeyBind {
            key: KeyCode::Char(']'),
            action: GlobalAction::TabNext,
            short_description: "Next tab",
            description: "Switch to the next tab in the current view",
        },
        GlobalKeyBind {
            key: KeyCode::Char('['),
            action: GlobalAction::TabPrev,
            short_description: "Previous tab",
            description: "Switch to the previous tab in the current view",
        },
        GlobalKeyBind {
            key: KeyCode::Char('s'),
            action: GlobalAction::CycleSort,
            short_description: "Sort",
            description: "Cycle through available sorting options",
        },
        GlobalKeyBind {
            key: KeyCode::Up,
            action: GlobalAction::Up,
            short_description: "Move up",
            description: "Move selection up in the current list",
        },
        GlobalKeyBind {
            key: KeyCode::Char('k'),
            action: GlobalAction::Up,
            short_description: "Move up",
            description: "Move selection up in the current list (vim-style)",
        },
        GlobalKeyBind {
            key: KeyCode::Down,
            action: GlobalAction::Down,
            short_description: "Move down",
            description: "Move selection down in the current list",
        },
        GlobalKeyBind {
            key: KeyCode::Char('j'),
            action: GlobalAction::Down,
            short_description: "Move down",
            description: "Move selection down in the current list (vim-style)",
        },
        GlobalKeyBind {
            key: KeyCode::Left,
            action: GlobalAction::Left,
            short_description: "Focus left",
            description: "Move focus to the left panel",
        },
        GlobalKeyBind {
            key: KeyCode::Char('h'),
            action: GlobalAction::Left,
            short_description: "Focus left",
            description: "Move focus to the left panel (vim-style)",
        },
        GlobalKeyBind {
            key: KeyCode::Right,
            action: GlobalAction::Right,
            short_description: "Focus right",
            description: "Move focus to the right panel",
        },
        GlobalKeyBind {
            key: KeyCode::Char('l'),
            action: GlobalAction::Right,
            short_description: "Focus right",
            description: "Move focus to the right panel (vim-style)",
        },
    ]
});

pub static DETAIL_KEYBINDS: Lazy<Vec<DetailKeyBind>> = Lazy::new(|| {
    vec![
        DetailKeyBind {
            key: KeyCode::Up,
            action: DetailAction::Up,
            short_description: "Previous field",
            description: "Navigate to the previous field in detail view",
        },
        DetailKeyBind {
            key: KeyCode::Char('k'),
            action: DetailAction::Up,
            short_description: "Previous field",
            description: "Navigate to the previous field in detail view (vim-style)",
        },
        DetailKeyBind {
            key: KeyCode::Down,
            action: DetailAction::Down,
            short_description: "Next field",
            description: "Navigate to the next field in detail view",
        },
        DetailKeyBind {
            key: KeyCode::Char('j'),
            action: DetailAction::Down,
            short_description: "Next field",
            description: "Navigate to the next field in detail view (vim-style)",
        },
        DetailKeyBind {
            key: KeyCode::Char('e'),
            action: DetailAction::Edit,
            short_description: "Edit",
            description: "Enter edit mode for the current field",
        },
        DetailKeyBind {
            key: KeyCode::Esc,
            action: DetailAction::Esc,
            short_description: "Exit/Cancel",
            description: "Exit edit mode or return focus to issue list",
        },
    ]
});
