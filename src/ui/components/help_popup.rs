use ratatui::{
    layout::Margin,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::keybinds::{DETAIL_KEYBINDS, GLOBAL_KEYBINDS};
use crate::app::state::HelpState;

use crate::ui::components::popup::popup_area;

pub fn draw_help_popup(show: bool, help_state: &HelpState, frame: &mut Frame) {
    if show {
        let area = frame.area();
        let popup_area = popup_area(area, 70, 70);

        frame.render_widget(Clear, popup_area);

        let block = Block::bordered()
            .title("Help - Use j/k or ↑/↓ to navigate")
            .border_style(Style::default().fg(Color::Yellow));
        frame.render_widget(block, popup_area);

        let mut lines = Vec::new();
        let mut current_index = 0;

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

        lines.push(Line::from("Global Keybindings:".bold().fg(Color::Cyan)));
        lines.push(Line::from(""));

        for bind in GLOBAL_KEYBINDS.iter() {
            let is_selected = current_index == help_state.selected_index;
            let key_str = format_key_code(&bind.key);
            let available_width = popup_area
                .inner(Margin {
                    vertical: 1,
                    horizontal: 2,
                })
                .width as usize;

            lines.push(create_help_line_normal(
                key_str,
                bind.short_description,
                bind.description,
                is_selected,
                available_width,
                max_key_width,
            ));

            lines.push(create_help_description_line(bind.description, is_selected));

            lines.push(Line::from(""));

            current_index += 1;
        }

        lines.push(Line::from(""));
        lines.push(Line::from(
            "Detail View Keybindings:".bold().fg(Color::Cyan),
        ));
        lines.push(Line::from(""));

        for bind in DETAIL_KEYBINDS.iter() {
            let is_selected = current_index == help_state.selected_index;
            let key_str = format_key_code(&bind.key);
            let available_width = popup_area
                .inner(Margin {
                    vertical: 1,
                    horizontal: 2,
                })
                .width as usize;

            lines.push(create_help_line_normal(
                key_str,
                bind.short_description,
                bind.description,
                is_selected,
                available_width,
                max_key_width,
            ));

            lines.push(create_help_description_line(bind.description, is_selected));

            lines.push(Line::from(""));

            current_index += 1;
        }

        let help_text = Text::from(lines);
        let help_paragraph = Paragraph::new(help_text)
            .wrap(Wrap { trim: true })
            .scroll((help_state.scroll_offset as u16, 0));

        let inner_content_area = popup_area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        });

        frame.render_widget(help_paragraph, inner_content_area);
    }
}

pub fn format_key_code(key: &crossterm::event::KeyCode) -> String {
    match key {
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        crossterm::event::KeyCode::Enter => "Enter".to_string(),
        crossterm::event::KeyCode::Esc => "Esc".to_string(),
        crossterm::event::KeyCode::Up => "↑".to_string(),
        crossterm::event::KeyCode::Down => "↓".to_string(),
        crossterm::event::KeyCode::Left => "←".to_string(),
        crossterm::event::KeyCode::Right => "→".to_string(),
        _ => format!("{:?}", key),
    }
}

pub fn create_help_line_normal(
    key: String,
    short_desc: &str,
    _long_desc: &str,
    is_selected: bool,
    available_width: usize,
    max_key_width: usize,
) -> Line<'static> {
    let selection_style = if is_selected {
        Style::default().bg(Color::Blue).fg(Color::White)
    } else {
        Style::default().fg(Color::White)
    };

    let key_style = if is_selected {
        Style::default().bg(Color::Blue).fg(Color::Yellow).bold()
    } else {
        Style::default().fg(Color::Yellow).bold()
    };

    let key_text = format!("[{}]", key);
    let key_text_padded = format!("{:>width$}", key_text, width = max_key_width);

    let prefix = "  ";
    let used_width = prefix.len() + short_desc.len() + max_key_width;
    let padding = if used_width < available_width {
        " ".repeat(available_width.saturating_sub(used_width))
    } else {
        " ".to_string()
    };

    Line::from(vec![
        Span::styled(prefix.to_string(), selection_style),
        Span::styled(short_desc.to_string(), selection_style),
        Span::styled(padding, selection_style),
        Span::styled(key_text_padded, key_style),
    ])
}

pub fn create_help_description_line(long_desc: &str, is_selected: bool) -> Line<'static> {
    let description_style = if is_selected {
        Style::default().bg(Color::Blue).fg(Color::LightBlue)
    } else {
        Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)
    };

    Line::from(vec![Span::styled(
        format!("    {}", long_desc),
        description_style,
    )])
}
