use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{
    services::{
        sort::SortMode,
        types::{JiraIssue, Paginated},
    },
    ui::components::{IssueItemMode, IssueItemRenderer},
};

pub mod icons;
mod navigation;
mod spinner;
mod tabs;

pub use navigation::{ListAction, ListNavigator};
pub use tabs::{JqlTab, TabAction, TabBar};

use spinner::LoadingSpinner;

pub struct IssueList {
    pub title: String,
    pub result: Paginated<JiraIssue>,
    pub focused: bool,
    pub sort_mode: SortMode,
    pub is_loading: bool,
    pub summary_mode: bool,
    pub tabs_enabled: bool,

    navigator: ListNavigator,
    spinner: LoadingSpinner,
    tab_bar: TabBar,
    issue_renderer: IssueItemRenderer,
}

impl IssueList {
    pub fn new(title: String, summary_mode: bool, sort_mode: SortMode) -> Self {
        Self {
            title,
            result: Paginated::new(),
            focused: false,
            sort_mode,
            is_loading: false,
            navigator: ListNavigator::new(),
            spinner: LoadingSpinner::new(),
            summary_mode,
            tabs_enabled: false,
            tab_bar: TabBar::new(),
            issue_renderer: IssueItemRenderer::new(),
        }
    }

    pub fn with_tabs(mut self, tabs: Vec<JqlTab>) -> Self {
        self.tab_bar = TabBar::with_tabs(tabs);
        self.tabs_enabled = true;
        self
    }

    #[allow(dead_code)]
    pub fn enable_tabs(&mut self) {
        self.tabs_enabled = true;
        if self.tab_bar.tabs.is_empty() {
            self.tab_bar = TabBar::default();
        }
    }

    #[allow(dead_code)]
    pub fn disable_tabs(&mut self) {
        self.tabs_enabled = false;
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) {
        self.focused = focused;

        if self.is_loading {
            self.spinner.advance();
        }

        let current_selection = self.navigator.selected().unwrap_or(0);
        let item_count = self.result.items.len();

        let items: Vec<ListItem> = self
            .result
            .items
            .iter()
            .map(|issue| {
                let mode = if self.summary_mode {
                    IssueItemMode::Summary
                } else {
                    IssueItemMode::Full
                };
                self.issue_renderer.render_issue(issue, mode, area.width)
            })
            .collect();

        let highlight_style = if self.focused {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let title = self.title.to_string();

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(if self.focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            });

        if self.focused {
            let sort_label = self.sort_mode.label();
            let top_right = if self.tabs_enabled {
                if let Some(tab) = self.tab_bar.current_tab() {
                    format!(" {} · {} ", tab.name, sort_label)
                } else {
                    format!(" {} ", sort_label)
                }
            } else {
                format!(" {} ", sort_label)
            };

            block = block.title_top(
                Line::from(Span::styled(top_right, Style::default().fg(Color::Yellow)))
                    .right_aligned(),
            );
        }

        if self.focused {
            let spinner_text = if self.is_loading {
                format!("{} ", self.spinner.current_frame())
            } else {
                "".to_string()
            };

            let selection_info =
                format!("{}{}/{}", spinner_text, current_selection + 1, item_count);

            block = block.title_bottom(Line::from(format!(" {} ", selection_info)).right_aligned());
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style);

        f.render_stateful_widget(list, area, self.navigator.state_mut());
    }

    pub fn move_up(&mut self) -> ListAction {
        self.navigator.move_up(self.result.items.len())
    }

    pub fn move_down(&mut self) -> ListAction {
        self.navigator.move_down(
            self.result.items.len(),
            self.result.has_more,
            self.is_loading,
        )
    }

    pub fn cycle_sort(&mut self) -> ListAction {
        self.sort_mode = self.sort_mode.next();
        ListAction::Sort
    }

    #[allow(dead_code)]
    pub fn state_selected(&self) -> Option<usize> {
        self.navigator.selected()
    }

    #[allow(dead_code)]
    pub fn select_item(&mut self, index: Option<usize>) {
        self.navigator.state_mut().select(index);
    }

    pub fn has_selection(&self) -> bool {
        self.navigator.selected().is_some()
    }

    pub fn ensure_selection(&mut self) {
        if self.navigator.selected().is_none() && !self.result.items.is_empty() {
            self.navigator.state_mut().select(Some(0));
        }
    }

    #[allow(dead_code)]
    pub fn sort_items(&mut self) {
        self.ensure_selection();
    }

    pub fn move_tab_left(&mut self) -> TabAction {
        if self.tabs_enabled {
            let old_index = self.tab_bar.selected_index;
            self.tab_bar.move_left();
            if old_index != self.tab_bar.selected_index {
                TabAction::TabChanged
            } else {
                TabAction::NoAction
            }
        } else {
            TabAction::NoAction
        }
    }

    pub fn move_tab_right(&mut self) -> TabAction {
        if self.tabs_enabled {
            let old_index = self.tab_bar.selected_index;
            self.tab_bar.move_right();
            if old_index != self.tab_bar.selected_index {
                TabAction::TabChanged
            } else {
                TabAction::NoAction
            }
        } else {
            TabAction::NoAction
        }
    }

    pub fn current_jql(&self) -> Option<String> {
        if self.tabs_enabled {
            self.tab_bar.current_jql().map(String::from)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn current_tab(&self) -> Option<&JqlTab> {
        if self.tabs_enabled {
            self.tab_bar.current_tab()
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn add_tab(&mut self, tab: JqlTab) {
        self.tab_bar.add_tab(tab);
        if !self.tabs_enabled {
            self.tabs_enabled = true;
        }
    }

    #[allow(dead_code)]
    pub fn tabs(&self) -> &[JqlTab] {
        &self.tab_bar.tabs
    }
}
