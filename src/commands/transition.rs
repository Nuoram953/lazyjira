use super::{Command, CommandResult};
use crate::context::AppContext;
use async_trait::async_trait;
use color_eyre::Result;
use log::{error, info};

pub struct ShowTransitionsCommand;

#[async_trait]
impl Command for ShowTransitionsCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        if let Some(issue_key) = ctx.get_selected_issue_key() {
            if ctx.use_api {
                if let Err(e) = load_transitions_for_issue(ctx, &issue_key).await {
                    error!("Failed to load transitions for issue {}: {}", issue_key, e);
                } else {
                    ctx.transitioning_issue_key = Some(issue_key);
                    ctx.navigation.show_transitions();
                }
            }
        }
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        "Show available transitions for selected issue"
    }
}

pub struct ExecuteTransitionCommand {
    transition_id: String,
}

impl ExecuteTransitionCommand {
    pub fn new(transition_id: String) -> Self {
        Self { transition_id }
    }
}

#[async_trait]
impl Command for ExecuteTransitionCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        if let Some(issue_key) = &ctx.transitioning_issue_key {
            if ctx.use_api {
                if let Err(e) = execute_transition(ctx, issue_key, &self.transition_id).await {
                    error!(
                        "Failed to execute transition {} for issue {}: {}",
                        self.transition_id, issue_key, e
                    );
                } else {
                    ctx.navigation.hide_transitions();
                    ctx.available_transitions.clear();
                    ctx.transitioning_issue_key = None;

                    ctx.refresh_data().await;
                }
            }
        }
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        "Execute transition for issue"
    }
}

async fn load_transitions_for_issue(ctx: &mut AppContext, issue_key: &str) -> Result<()> {
    if let Some(data_manager) = &ctx.data_manager {
        match data_manager.get_issue_transitions(issue_key).await {
            Ok(transitions) => {
                ctx.available_transitions = transitions;
                info!(
                    "Loaded {} transitions for issue {}",
                    ctx.available_transitions.len(),
                    issue_key
                );
                Ok(())
            }
            Err(e) => {
                error!("Failed to load transitions for issue {}: {}", issue_key, e);
                Err(color_eyre::eyre::eyre!("Failed to load transitions: {}", e))
            }
        }
    } else {
        Err(color_eyre::eyre::eyre!("No API client available"))
    }
}

async fn execute_transition(ctx: &AppContext, issue_key: &str, transition_id: &str) -> Result<()> {
    if let Some(data_manager) = &ctx.data_manager {
        match data_manager
            .transition_issue(issue_key, transition_id)
            .await
        {
            Ok(()) => {
                info!(
                    "Successfully transitioned issue {} with transition {}",
                    issue_key, transition_id
                );
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to execute transition {} for issue {}: {}",
                    transition_id, issue_key, e
                );
                Err(color_eyre::eyre::eyre!(
                    "Failed to execute transition: {}",
                    e
                ))
            }
        }
    } else {
        Err(color_eyre::eyre::eyre!("No API client available"))
    }
}
