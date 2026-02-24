use crossterm::event::KeyCode;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKey {
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
}

pub struct KeyBind {
    pub key: KeyCode,
    pub action: ActionKey,
    #[allow(dead_code)]
    pub description: &'static str,
}

pub static GLOBAL_KEYBINDS: Lazy<Vec<KeyBind>> = Lazy::new(|| {
    vec![
        KeyBind {
            key: KeyCode::Char('q'),
            action: ActionKey::Quit,
            description: "Quit the app",
        },
        KeyBind {
            key: KeyCode::Char('/'),
            action: ActionKey::Search,
            description: "Enter search mode",
        },
        KeyBind {
            key: KeyCode::Esc,
            action: ActionKey::Esc,
            description: "Cancel / exit search",
        },
        KeyBind {
            key: KeyCode::Char(']'),
            action: ActionKey::TabNext,
            description: "Next tab",
        },
        KeyBind {
            key: KeyCode::Char('['),
            action: ActionKey::TabPrev,
            description: "Previous tab",
        },
        KeyBind {
            key: KeyCode::Char('s'),
            action: ActionKey::CycleSort,
            description: "Cycle sort",
        },
        KeyBind {
            key: KeyCode::Up,
            action: ActionKey::Up,
            description: "Move selection up",
        },
        KeyBind {
            key: KeyCode::Char('k'),
            action: ActionKey::Up,
            description: "Move selection up",
        },
        KeyBind {
            key: KeyCode::Down,
            action: ActionKey::Down,
            description: "Move selection down",
        },
        KeyBind {
            key: KeyCode::Char('j'),
            action: ActionKey::Down,
            description: "Move selection down",
        },
        KeyBind {
            key: KeyCode::Left,
            action: ActionKey::Left,
            description: "Focus",
        },
        KeyBind {
            key: KeyCode::Char('h'),
            action: ActionKey::Left,
            description: "Focus",
        },
        KeyBind {
            key: KeyCode::Right,
            action: ActionKey::Right,
            description: "Focus",
        },
        KeyBind {
            key: KeyCode::Char('l'),
            action: ActionKey::Right,
            description: "Focus",
        },
    ]
});
