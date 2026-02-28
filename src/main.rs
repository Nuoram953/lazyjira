use dotenv::dotenv;
use tokio::{select, sync::mpsc};

mod app;
mod core;
mod services;
mod ui;

use app::{App, AppAction, AppMessage};

use crate::{app::ActiveList, services::sort::SortMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;

    dotenv().ok();
    let mut terminal = core::terminal::init_terminal()?;

    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut app = App::new(tx.clone());

    let tx = app.tx.clone();

    //temporary - load initial data for all lists
    let sprint_items = app
        .client
        .fetch_current_sprint_issues(SortMode::PriorityDesc, Some("".to_string()), 0)
        .await
        .unwrap_or_default();
    let _ = tx.send(AppMessage::ItemsLoaded {
        list: ActiveList::Sprint,
        result: sprint_items,
        append: false,
    });

    let recently_updated_items = app
        .client
        .fetch_recently_updated_issues(SortMode::UpdatedDesc, Some("".to_string()), 0)
        .await
        .unwrap_or_default();
    let _ = tx.send(AppMessage::ItemsLoaded {
        list: ActiveList::RecentlyUpdated,
        result: recently_updated_items,
        append: false,
    });

    let backlog_items = app
        .client
        .fetch_backlog_issues(SortMode::KeyDesc, Some("".to_string()), 0)
        .await
        .unwrap_or_default();
    let _ = tx.send(AppMessage::ItemsLoaded {
        list: ActiveList::Backlog,
        result: backlog_items,
        append: false,
    });

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        select! {
            Ok(event) = core::event::next_event() => {
                if let Some(action) = app.handle_event(event) {
                    handle_action(action, &mut app).await;
                }
            }

            Some(msg) = rx.recv() => {
                app.update_from_message(msg);
            }
        }

        if app.should_quit {
            break;
        }
    }

    core::terminal::restore_terminal(&mut terminal)?;
    Ok(())
}

async fn handle_action(action: AppAction, app: &mut App) {
    match action {
        AppAction::Quit => app.should_quit = true,

        AppAction::LoadItems => {
            if app.loading {
                return;
            }

            app.loading = true;
            let tx = app.tx.clone();
            let active_list = app.navigator.active;
            let client = app.client.clone();

            let filter = match active_list {
                ActiveList::Sprint => app.items_sprint.current_jql(),
                ActiveList::RecentlyUpdated => app.items_recently_updated.current_jql(),
                ActiveList::Backlog => app.items_backlog.current_jql(),
            };

            let sort = match active_list {
                ActiveList::Sprint => app.items_sprint.sort_mode,
                ActiveList::RecentlyUpdated => app.items_recently_updated.sort_mode,
                ActiveList::Backlog => app.items_backlog.sort_mode,
            };

            let page = match active_list {
                ActiveList::Sprint => app.items_sprint.result.page,
                ActiveList::RecentlyUpdated => app.items_recently_updated.result.page,
                ActiveList::Backlog => app.items_backlog.result.page,
            };

            tokio::spawn(async move {
                let result = match active_list {
                    ActiveList::Sprint => {
                        client.fetch_current_sprint_issues(sort, filter, page).await
                    }
                    ActiveList::RecentlyUpdated => {
                        client
                            .fetch_recently_updated_issues(sort, filter, page)
                            .await
                    }
                    ActiveList::Backlog => client.fetch_backlog_issues(sort, filter, page).await,
                };

                match result {
                    Ok(items) => {
                        let _ = tx.send(AppMessage::ItemsLoaded {
                            list: active_list,
                            result: items,
                            append: true,
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(AppMessage::Error {
                            list: active_list,
                            message: e.to_string(),
                        });
                    }
                }
            });
        }
    }
}

fn init_logging() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
