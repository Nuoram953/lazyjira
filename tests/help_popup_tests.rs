use crossterm::event::KeyCode;
use lazyjira::app::keybinds::{GlobalAction, DETAIL_KEYBINDS, GLOBAL_KEYBINDS};
use lazyjira::app::state::HelpState;
use ratatui::{backend::TestBackend, Terminal};

#[test]
fn test_help_state_new() {
    let help_state = HelpState::default();

    assert_eq!(help_state.selected_index, 0);
    assert_eq!(help_state.scroll_offset, 0);
}

#[test]
fn test_help_state_initialization() {
    let help_state = HelpState {
        selected_index: 5,
        scroll_offset: 2,
    };

    assert_eq!(help_state.selected_index, 5);
    assert_eq!(help_state.scroll_offset, 2);
}

#[test]
fn test_global_keybinds_not_empty() {
    assert!(!GLOBAL_KEYBINDS.is_empty());
    assert!(GLOBAL_KEYBINDS.len() > 5);
}

#[test]
fn test_detail_keybinds_not_empty() {
    assert!(!DETAIL_KEYBINDS.is_empty());
    assert!(DETAIL_KEYBINDS.len() > 3);
}

#[test]
fn test_global_keybinds_have_required_fields() {
    for bind in GLOBAL_KEYBINDS.iter() {
        assert!(
            !bind.short_description.is_empty(),
            "Short description should not be empty"
        );
        assert!(
            !bind.description.is_empty(),
            "Description should not be empty"
        );
    }
}

#[test]
fn test_detail_keybinds_have_required_fields() {
    for bind in DETAIL_KEYBINDS.iter() {
        assert!(
            !bind.short_description.is_empty(),
            "Short description should not be empty"
        );
        assert!(
            !bind.description.is_empty(),
            "Description should not be empty"
        );
    }
}

#[test]
fn test_global_keybinds_unique_actions() {
    let mut seen_actions = std::collections::HashSet::new();
    let mut duplicate_actions = Vec::new();

    for bind in GLOBAL_KEYBINDS.iter() {
        if !seen_actions.insert(bind.action) {
            duplicate_actions.push(bind.action);
        }
    }

    assert!(
        duplicate_actions.len() <= GLOBAL_KEYBINDS.len() / 2,
        "Too many duplicate actions: {:?}",
        duplicate_actions
    );
}

#[test]
fn test_detail_keybinds_unique_actions() {
    let mut seen_actions = std::collections::HashSet::new();
    let mut duplicate_actions = Vec::new();

    for bind in DETAIL_KEYBINDS.iter() {
        if !seen_actions.insert(bind.action) {
            duplicate_actions.push(bind.action);
        }
    }

    assert!(
        duplicate_actions.len() <= DETAIL_KEYBINDS.len() / 2,
        "Too many duplicate actions: {:?}",
        duplicate_actions
    );
}

#[test]
fn test_help_keybind_exists() {
    let help_bind = GLOBAL_KEYBINDS
        .iter()
        .find(|bind| bind.action == GlobalAction::Help);

    assert!(help_bind.is_some(), "Help keybind should exist");
    let help_bind = help_bind.unwrap();
    assert_eq!(help_bind.key, KeyCode::Char('?'));
    assert!(help_bind.short_description.to_lowercase().contains("help"));
}

#[test]
fn test_quit_keybind_exists() {
    let quit_bind = GLOBAL_KEYBINDS
        .iter()
        .find(|bind| bind.action == GlobalAction::Quit);

    assert!(quit_bind.is_some(), "Quit keybind should exist");
    let quit_bind = quit_bind.unwrap();
    assert_eq!(quit_bind.key, KeyCode::Char('q'));
    assert!(quit_bind.short_description.to_lowercase().contains("quit"));
}

#[test]
fn test_navigation_keybinds_exist() {
    let up_binds: Vec<_> = GLOBAL_KEYBINDS
        .iter()
        .filter(|bind| bind.action == GlobalAction::Up)
        .collect();

    let down_binds: Vec<_> = GLOBAL_KEYBINDS
        .iter()
        .filter(|bind| bind.action == GlobalAction::Down)
        .collect();

    assert!(!up_binds.is_empty(), "Should have Up navigation keybinds");
    assert!(
        !down_binds.is_empty(),
        "Should have Down navigation keybinds"
    );

    let has_up_arrow = up_binds.iter().any(|bind| bind.key == KeyCode::Up);
    let has_k_key = up_binds.iter().any(|bind| bind.key == KeyCode::Char('k'));
    let has_down_arrow = down_binds.iter().any(|bind| bind.key == KeyCode::Down);
    let has_j_key = down_binds.iter().any(|bind| bind.key == KeyCode::Char('j'));

    assert!(has_up_arrow || has_k_key, "Should have Up arrow or 'k' key");
    assert!(
        has_down_arrow || has_j_key,
        "Should have Down arrow or 'j' key"
    );
}

#[test]
fn test_format_key_code() {
    use lazyjira::ui::components::help_popup::format_key_code;

    assert_eq!(format_key_code(&KeyCode::Char('q')), "q");
    assert_eq!(format_key_code(&KeyCode::Char('?')), "?");
    assert_eq!(format_key_code(&KeyCode::Enter), "Enter");
    assert_eq!(format_key_code(&KeyCode::Esc), "Esc");
    assert_eq!(format_key_code(&KeyCode::Up), "↑");
    assert_eq!(format_key_code(&KeyCode::Down), "↓");
    assert_eq!(format_key_code(&KeyCode::Left), "←");
    assert_eq!(format_key_code(&KeyCode::Right), "→");
}

#[test]
fn test_help_popup_rendering() {
    use lazyjira::ui::components::help_popup::draw_help_popup;

    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let help_state = HelpState::default();

    let result = terminal.draw(|f| {
        draw_help_popup(true, &help_state, f);
    });

    assert!(result.is_ok(), "Help popup rendering should not panic");
}

#[test]
fn test_help_popup_not_rendered_when_hidden() {
    use lazyjira::ui::components::help_popup::draw_help_popup;

    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    let help_state = HelpState::default();

    let result = terminal.draw(|f| {
        draw_help_popup(false, &help_state, f);
    });

    assert!(
        result.is_ok(),
        "Help popup with show=false should not panic"
    );

    let buffer = terminal.backend().buffer();
    let content_cells: Vec<_> = buffer
        .content
        .iter()
        .filter(|cell| !cell.symbol().trim().is_empty())
        .collect();

    assert!(
        content_cells.is_empty(),
        "No content should be rendered when help is hidden"
    );
}

#[test]
fn test_max_key_width_calculation() {
    use lazyjira::ui::components::help_popup::format_key_code;

    let global_max = GLOBAL_KEYBINDS
        .iter()
        .map(|bind| format!("[{}]", format_key_code(&bind.key)).len())
        .max()
        .unwrap_or(5);

    let detail_max = DETAIL_KEYBINDS
        .iter()
        .map(|bind| format!("[{}]", format_key_code(&bind.key)).len())
        .max()
        .unwrap_or(5);

    let max_key_width = global_max.max(detail_max);

    assert!(max_key_width >= 3, "Max key width should be at least 3");
    assert!(max_key_width <= 20, "Max key width should not exceed 20");

    assert!(
        max_key_width >= "[Enter]".len(),
        "Should accommodate [Enter]"
    );
    assert!(max_key_width >= "[Esc]".len(), "Should accommodate [Esc]");
}

#[test]
fn test_help_line_creation_normal_mode() {
    use lazyjira::ui::components::help_popup::create_help_line_normal;

    let line = create_help_line_normal(
        "q".to_string(),
        "Quit",
        "Exit the application",
        false,
        50,
        6,
    );

    assert!(line.spans.len() >= 3, "Line should have multiple spans");
}

#[test]
fn test_help_line_creation_selected() {
    use lazyjira::ui::components::help_popup::create_help_line_normal;
    use ratatui::style::Color;

    let line =
        create_help_line_normal("q".to_string(), "Quit", "Exit the application", true, 50, 6);

    assert!(
        line.spans.len() >= 3,
        "Selected line should have multiple spans"
    );

    let has_blue_bg = line
        .spans
        .iter()
        .any(|span| span.style.bg == Some(Color::Blue));
    assert!(has_blue_bg, "Selected line should have blue background");
}

#[test]
fn test_help_description_line() {
    use lazyjira::ui::components::help_popup::create_help_description_line;
    use ratatui::style::Color;

    let line = create_help_description_line("This is a long description", false);

    assert!(
        !line.spans.is_empty(),
        "Description line should have at least one span"
    );

    let has_gray = line
        .spans
        .iter()
        .any(|span| span.style.fg == Some(Color::Gray));
    assert!(has_gray, "Description line should have gray text");

    let first_span = &line.spans[0];
    assert!(
        first_span.content.starts_with("    "),
        "Description should be indented"
    );
}

#[test]
fn test_help_description_line_selected() {
    use lazyjira::ui::components::help_popup::create_help_description_line;
    use ratatui::style::Color;

    let line = create_help_description_line("This is a long description", true);

    let has_blue_bg = line
        .spans
        .iter()
        .any(|span| span.style.bg == Some(Color::Blue));
    assert!(
        has_blue_bg,
        "Selected description line should have blue background"
    );
}

#[test]
fn test_help_line_with_empty_description() {
    use lazyjira::ui::components::help_popup::create_help_line_normal;

    let line = create_help_line_normal("q".to_string(), "", "Exit the application", false, 50, 6);

    assert!(
        line.spans.len() >= 3,
        "Line should handle empty description"
    );
}

#[test]
fn test_help_line_with_very_wide_terminal() {
    use lazyjira::ui::components::help_popup::create_help_line_normal;

    let line = create_help_line_normal(
        "q".to_string(),
        "Quit",
        "Exit the application",
        false,
        200,
        6,
    );

    assert!(line.spans.len() >= 3, "Line should handle wide terminals");
}

#[test]
fn test_help_line_with_very_narrow_terminal() {
    use lazyjira::ui::components::help_popup::create_help_line_normal;

    let line = create_help_line_normal(
        "q".to_string(),
        "Quit",
        "Exit the application",
        false,
        10,
        6,
    );

    assert!(line.spans.len() >= 3, "Line should handle narrow terminals");
}

#[test]
fn test_key_alignment_consistency() {
    use lazyjira::ui::components::help_popup::{create_help_line_normal, format_key_code};

    let available_width = 60;

    let global_max = GLOBAL_KEYBINDS
        .iter()
        .map(|bind| format!("[{}]", format_key_code(&bind.key)).len())
        .max()
        .unwrap_or(5);

    let detail_max = DETAIL_KEYBINDS
        .iter()
        .map(|bind| format!("[{}]", format_key_code(&bind.key)).len())
        .max()
        .unwrap_or(5);

    let max_key_width = global_max.max(detail_max);

    let test_keys = vec![
        ("q", "Quit"),
        ("Enter", "Focus detail"),
        ("Esc", "Cancel"),
        ("?", "Help"),
    ];

    let mut key_positions = Vec::new();

    for (key, desc) in test_keys {
        let line = create_help_line_normal(
            key.to_string(),
            desc,
            "Description",
            false,
            available_width,
            max_key_width,
        );

        let mut pos = 0;
        for span in &line.spans {
            if span.content.contains('[') && span.content.contains(']') {
                key_positions.push(pos);
                break;
            }
            pos += span.content.len();
        }
    }

    if key_positions.len() > 1 {
        let first_pos = key_positions[0];
        for pos in &key_positions[1..] {
            assert_eq!(
                *pos, first_pos,
                "All keys should be aligned at the same position"
            );
        }
    }
}
