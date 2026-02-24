use nerd_fonts::NerdFonts;
use std::collections::HashMap;

pub struct PriorityIcons {
    icons: HashMap<String, char>,
    default_icon: char,
}

impl PriorityIcons {
    pub fn new() -> Self {
        let nf = NerdFonts {
            nf: NerdFonts::load(),
        };

        let mut icons = HashMap::new();
        icons.insert(
            "highest".to_string(),
            nf.get("md-chevron_double_up").unwrap_or('!'),
        );
        icons.insert("high".to_string(), nf.get("md-chevron_up").unwrap_or('↑'));
        icons.insert("medium".to_string(), nf.get("md-equal").unwrap_or('='));
        icons.insert("low".to_string(), nf.get("md-chevron_down").unwrap_or('↓'));
        icons.insert(
            "lowest".to_string(),
            nf.get("md-chevron_double_down").unwrap_or('v'),
        );

        Self {
            icons,
            default_icon: '?',
        }
    }

    pub fn get_icon(&self, priority: Option<&String>) -> char {
        priority
            .map(|p| p.to_lowercase())
            .and_then(|p| self.icons.get(&p))
            .copied()
            .unwrap_or(self.default_icon)
    }
}

impl Default for PriorityIcons {
    fn default() -> Self {
        Self::new()
    }
}
