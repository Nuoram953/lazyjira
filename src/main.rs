use color_eyre::Result;
use crossterm::event::{self, Event};
use dotenv::dotenv;
use log::warn;
use std::time::Duration;

// Use the library crate instead of declaring modules twice
use lazyjira::api::{JiraClient, JiraConfig};
use lazyjira::data_manager::{DataManager, DataManagerConfig};
use lazyjira::models::AppData;
use lazyjira::navigation::{AppAction, AppView, Direction, NavigationState};
use lazyjira::tui;
use lazyjira::ui;
use lazyjira::{AppContext, CommandRegistry, CommandResult};

#[derive(Debug)]
pub struct App {
    context: AppContext,
    command_registry: CommandRegistry,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
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

        let data = if use_api {
            AppData::new()
        } else {
            AppData::with_mock_data()
        };

        let context = AppContext::new(data, NavigationState::new(), data_manager, use_api);

        Self {
            context,
            command_registry: CommandRegistry::new(),
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
            terminal.draw(|frame| {
                ui::UI::render_with_transitions(
                    frame,
                    &self.context.data,
                    &self.context.navigation,
                    &self.context.available_transitions,
                )
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    let action = self.handle_key_event(key);
                    let result = self
                        .command_registry
                        .execute_action(&action, &mut self.context)
                        .await?;

                    match result {
                        CommandResult::Quit => break,
                        CommandResult::Continue | CommandResult::Handled => {}
                    }
                }
            }

            if self.context.use_api && last_refresh.elapsed() > refresh_interval {
                self.refresh_data().await;
                last_refresh = std::time::Instant::now();
            }

            if self.context.should_quit {
                break;
            }
        }

        tui::restore()?;
        log::info!("Application terminated successfully");
        Ok(())
    }

    async fn load_initial_data(&mut self) -> Result<()> {
        if self.context.use_api {
            self.refresh_data().await;
        }
        Ok(())
    }

    async fn refresh_data(&mut self) {
        if let Some(data_manager) = &self.context.data_manager {
            match data_manager.refresh_all_data().await {
                Ok(_) => {
                    let new_data = data_manager.get_data().await;
                    let selected_issue_key = self
                        .context
                        .data
                        .selected_issue
                        .as_ref()
                        .map(|i| i.key.clone());
                    self.context.data = new_data;

                    if let Some(_key) = selected_issue_key {
                        // TODO: Restore selected issue after refresh
                    }
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

        match self.context.navigation.current_view {
            AppView::Main => match (key.code, key.modifiers) {
                (Char('q'), KeyModifiers::NONE) | (Char('c'), KeyModifiers::CONTROL) => {
                    AppAction::Quit
                }

                (Char('?'), KeyModifiers::NONE) => AppAction::ShowHelp,

                (Char('t'), KeyModifiers::CONTROL) => AppAction::ShowTransitions,

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
            AppView::TransitionSelector => match (key.code, key.modifiers) {
                (Char('q'), KeyModifiers::NONE) | (Char('c'), KeyModifiers::CONTROL) => {
                    AppAction::Quit
                }
                (Esc, KeyModifiers::NONE) => AppAction::GoBack,
                (Char('j'), KeyModifiers::NONE) | (Down, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Down)
                }
                (Char('k'), KeyModifiers::NONE) | (Up, KeyModifiers::NONE) => {
                    AppAction::Navigate(Direction::Up)
                }
                (Enter, KeyModifiers::NONE) => {
                    if !self.context.available_transitions.is_empty()
                        && self.context.navigation.transition_selected
                            < self.context.available_transitions.len()
                    {
                        let transition_id = self.context.available_transitions
                            [self.context.navigation.transition_selected]
                            .id
                            .clone();
                        AppAction::ExecuteTransition(transition_id)
                    } else {
                        AppAction::None
                    }
                }
                _ => AppAction::None,
            },
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
        .filter_level(log::LevelFilter::Debug)
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
