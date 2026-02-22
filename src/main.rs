use color_eyre::Result;
use crossterm::event::{self, Event};
use dotenv::dotenv;
use log::{info, trace, warn};
use std::time::Duration;

mod api;
mod data_manager;
mod jira;
mod models;
mod navigation;
mod tui;
mod ui;

use api::JiraClient;
use data_manager::{DataManager, DataManagerConfig};
use models::AppData;
use navigation::{AppAction, AppView, Direction, FocusedPane, NavigationState};

use crate::jira::JiraConfig;

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub data: AppData,
    pub navigation: NavigationState,
    pub data_manager: Option<DataManager>,
    pub use_api: bool,
}

impl App {
    pub fn new() -> Self {
        let (data_manager, use_api) = match Self::create_data_manager() {
            Ok(dm) => (Some(dm), true),
            Err(e) => {
                warn!("Could not initialize Jira API client: {}", e);
                warn!("Using mock data. Set JIRA_BASE_URL, JIRA_USERNAME, and JIRA_API_TOKEN environment variables to use real data.");
                (None, false)
            }
        };

        Self {
            should_quit: false,
            data: if use_api {
                AppData::new()
            } else {
                AppData::with_mock_data()
            },
            navigation: NavigationState::new(),
            data_manager,
            use_api,
        }
    }

    fn create_data_manager() -> color_eyre::Result<DataManager> {
        let config = JiraConfig::from_env()?;

        log::debug!("Jira config: {:?}", config);

        if config.base_url.contains("your-domain")
            || config.username.contains("your-email")
            || config.api_token.contains("your-api-token")
        {
            return Err(color_eyre::eyre::eyre!(
                "Please configure your Jira credentials in environment variables"
            ));
        }

        let client = JiraClient::new(
            config.base_url.clone(),
            config.username.clone(),
            config.api_token.clone(),
        )?;

        let data_manager_config = DataManagerConfig {
            board_id: config.board_id.clone(),
            max_results: config.max_results,
            auto_refresh_interval: Some(std::time::Duration::from_secs(60)),
        };

        Ok(DataManager::new(client, data_manager_config))
    }

    pub async fn run(&mut self) -> Result<()> {
        log::info!("Starting TUI application");
        let mut terminal = tui::init()?;
        log::info!("Terminal initialized successfully");

        self.load_initial_data().await?;

        let mut last_refresh = std::time::Instant::now();
        let refresh_interval = std::time::Duration::from_secs(30);

        loop {
            terminal.draw(|frame| ui::UI::render(frame, &self.data, &self.navigation))?;

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) => {
                        let action = self.handle_key_event(key);
                        self.handle_action(action).await?;
                    }
                    _ => {}
                }
            }

            if self.use_api && last_refresh.elapsed() > refresh_interval {
                self.refresh_data().await;
                last_refresh = std::time::Instant::now();
            }

            if self.should_quit {
                break;
            }
        }

        tui::restore()?;
        Ok(())
    }

    async fn load_initial_data(&mut self) -> Result<()> {
        info!("{}", "load_initial_data");
        if let Some(data_manager) = &self.data_manager {
            match data_manager.refresh_all_data().await {
                Ok(_) => {
                    self.data = data_manager.get_data().await;
                    log::info!("Successfully loaded data from Jira API");
                }
                Err(e) => {
                    log::error!(
                        "Error loading initial data from API: {}. Using mock data.",
                        e
                    );
                    self.data = AppData::with_mock_data();
                    self.use_api = false;
                }
            }
        }
        Ok(())
    }

    async fn refresh_data(&mut self) {
        if let Some(data_manager) = &self.data_manager {
            match data_manager.refresh_all_data().await {
                Ok(_) => {
                    self.data = data_manager.get_data().await;
                }
                Err(e) => {
                    log::error!("Error refreshing data: {}", e);
                }
            }
        }
    }

    fn handle_key_event(&self, key: crossterm::event::KeyEvent) -> AppAction {
        use crossterm::event::KeyCode::*;
        use crossterm::event::KeyModifiers;

        match self.navigation.current_view {
            AppView::Main => match (key.code, key.modifiers) {
                (Char('q'), KeyModifiers::NONE) | (Char('c'), KeyModifiers::CONTROL) => {
                    AppAction::Quit
                }

                (Char('?'), KeyModifiers::NONE) => AppAction::ShowHelp,

                (Char('h'), KeyModifiers::NONE) | (Left, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Left)
                }
                (Char('j'), KeyModifiers::NONE) | (Down, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Down)
                }
                (Char('k'), KeyModifiers::NONE) | (Up, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Up)
                }
                (Char('l'), KeyModifiers::NONE) | (Right, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Right)
                }

                (Char('H'), KeyModifiers::NONE) => AppAction::Navigate(Direction::Left),
                (Char('L'), KeyModifiers::NONE) => AppAction::Navigate(Direction::Right),

                (Enter, KeyModifiers::NONE) => AppAction::SelectItem,

                (Esc, KeyModifiers::NONE) => AppAction::GoBack,

                _ => AppAction::None,
            },
            AppView::Help => match (key.code, key.modifiers) {
                (Char('q'), KeyModifiers::NONE) | (Char('c'), KeyModifiers::CONTROL) => {
                    AppAction::Quit
                }
                (Char('?'), KeyModifiers::NONE) | (Esc, KeyModifiers::NONE) => AppAction::GoBack,
                _ => AppAction::None,
            },
        }
    }

    async fn handle_action(&mut self, action: AppAction) -> Result<()> {
        match action {
            AppAction::Quit => {
                self.should_quit = true;
            }
            AppAction::ShowHelp => {
                self.navigation.current_view = AppView::Help;
            }
            AppAction::Navigate(direction) => {
                self.handle_navigation(direction);
            }
            AppAction::SelectItem => {
                self.handle_selection();
            }
            AppAction::GoBack => match self.navigation.current_view {
                AppView::Help => {
                    self.navigation.current_view = AppView::Main;
                }
                AppView::Main => {
                    if self.navigation.focused_pane == FocusedPane::Detail {
                        self.navigation.go_back_from_detail();
                    }
                }
            },
            AppAction::None => {}
        }
        Ok(())
    }

    fn handle_navigation(&mut self, direction: Direction) {
        match direction {
            Direction::Left | Direction::Right => {
                self.navigation.move_focus(direction);
            }
            Direction::Up | Direction::Down => {
                if self.navigation.focused_pane == FocusedPane::Detail {
                    self.navigation.move_focus(direction);
                } else {
                    let max_items = match self.navigation.focused_pane {
                        FocusedPane::Sprint => {
                            if let Some(sprint) = &self.data.current_sprint {
                                sprint.issues.len()
                            } else {
                                0
                            }
                        }
                        FocusedPane::Board => self.data.board_issues.len(),
                        FocusedPane::LastUpdated => self.data.last_updated_issues.len(),
                        FocusedPane::Detail => 0,
                    };

                    self.navigation.move_selection(direction, max_items);
                    self.update_selected_issue();
                }
            }
        }
    }

    fn handle_selection(&mut self) {
        match self.navigation.focused_pane {
            FocusedPane::Sprint | FocusedPane::Board | FocusedPane::LastUpdated => {
                self.update_selected_issue();

                self.navigation.focus_detail();
            }
            FocusedPane::Detail => {}
        }
    }

    fn update_selected_issue(&mut self) {
        match self.navigation.focused_pane {
            FocusedPane::Sprint => {
                if let Some(sprint) = &self.data.current_sprint {
                    if let Some(issue) = sprint.issues.get(self.navigation.sprint_selected) {
                        self.data.selected_issue = Some(issue.clone());
                    }
                }
            }
            FocusedPane::Board => {
                if let Some(issue) = self
                    .data
                    .board_issues
                    .get(self.navigation.last_viewed_selected)
                {
                    self.data.selected_issue = Some(issue.clone());
                }
            }
            FocusedPane::LastUpdated => {
                if let Some(issue) = self
                    .data
                    .last_updated_issues
                    .get(self.navigation.last_updated_selected)
                {
                    self.data.selected_issue = Some(issue.clone());
                }
            }
            FocusedPane::Detail => {}
        }
    }
}

fn init_logging() -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    std::fs::create_dir_all("logs")?;

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("logs/lazyjira.log")?,
        )))
        .filter_level(log::LevelFilter::Debug) // Enable debug logging
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    log::info!("Logging initialized - writing to logs/lazyjira.log");
    log::info!("Log level set to DEBUG - you can control this with RUST_LOG environment variable");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    dotenv().ok();

    init_logging()?;

    let mut app = App::new();
    app.run().await?;

    Ok(())
}
