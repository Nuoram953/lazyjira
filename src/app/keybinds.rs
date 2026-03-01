use crossterm::event::KeyCode;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlobalAction {
    Quit,
    Search,
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
    #[allow(dead_code)]
    pub description: &'static str,
}

pub struct DetailKeyBind {
    pub key: KeyCode,
    pub action: DetailAction,
    #[allow(dead_code)]
    pub description: &'static str,
}

pub static GLOBAL_KEYBINDS: Lazy<Vec<GlobalKeyBind>> = Lazy::new(|| {
    vec![
        GlobalKeyBind {
            key: KeyCode::Enter,
            action: GlobalAction::Enter,
            description: "Focused the detail pane",
        },
        GlobalKeyBind {
            key: KeyCode::Char('q'),
            action: GlobalAction::Quit,
            description: "Quit the app",
        },
        GlobalKeyBind {
            key: KeyCode::Char('/'),
            action: GlobalAction::Search,
            description: "Enter search mode",
        },
        GlobalKeyBind {
            key: KeyCode::Esc,
            action: GlobalAction::Esc,
            description: "Cancel / exit search",
        },
        GlobalKeyBind {
            key: KeyCode::Char(']'),
            action: GlobalAction::TabNext,
            description: "Next tab",
        },
        GlobalKeyBind {
            key: KeyCode::Char('['),
            action: GlobalAction::TabPrev,
            description: "Previous tab",
        },
        GlobalKeyBind {
            key: KeyCode::Char('s'),
            action: GlobalAction::CycleSort,
            description: "Cycle sort",
        },
        GlobalKeyBind {
            key: KeyCode::Up,
            action: GlobalAction::Up,
            description: "Move selection up",
        },
        GlobalKeyBind {
            key: KeyCode::Char('k'),
            action: GlobalAction::Up,
            description: "Move selection up",
        },
        GlobalKeyBind {
            key: KeyCode::Down,
            action: GlobalAction::Down,
            description: "Move selection down",
        },
        GlobalKeyBind {
            key: KeyCode::Char('j'),
            action: GlobalAction::Down,
            description: "Move selection down",
        },
        GlobalKeyBind {
            key: KeyCode::Left,
            action: GlobalAction::Left,
            description: "Focus",
        },
        GlobalKeyBind {
            key: KeyCode::Char('h'),
            action: GlobalAction::Left,
            description: "Focus",
        },
        GlobalKeyBind {
            key: KeyCode::Right,
            action: GlobalAction::Right,
            description: "Focus",
        },
        GlobalKeyBind {
            key: KeyCode::Char('l'),
            action: GlobalAction::Right,
            description: "Focus",
        },
    ]
});

pub static DETAIL_KEYBINDS: Lazy<Vec<DetailKeyBind>> = Lazy::new(|| {
    vec![
        DetailKeyBind {
            key: KeyCode::Up,
            action: DetailAction::Up,
            description: "Move to previous field",
        },
        DetailKeyBind {
            key: KeyCode::Char('k'),
            action: DetailAction::Up,
            description: "Move to previous field",
        },
        DetailKeyBind {
            key: KeyCode::Down,
            action: DetailAction::Down,
            description: "Move to next field",
        },
        DetailKeyBind {
            key: KeyCode::Char('j'),
            action: DetailAction::Down,
            description: "Move to next field",
        },
        DetailKeyBind {
            key: KeyCode::Char('e'),
            action: DetailAction::Edit,
            description: "Edit field",
        },
        DetailKeyBind {
            key: KeyCode::Esc,
            action: DetailAction::Esc,
            description: "Exit edit mode or detail focus",
        },
    ]
});
