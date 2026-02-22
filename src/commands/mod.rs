pub mod navigation;
pub mod transition;
pub mod ui;

use crate::context::AppContext;
use async_trait::async_trait;
use color_eyre::Result;

#[derive(Debug)]
pub enum CommandResult {
    Continue,

    Quit,

    Handled,
}

#[async_trait]
pub trait Command: Send + Sync {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult>;

    fn description(&self) -> &str;
}

#[derive(Debug)]
pub struct CommandRegistry {}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute_action(
        &self,
        action: &AppAction,
        ctx: &mut AppContext,
    ) -> Result<CommandResult> {
        match action {
            AppAction::Quit => ui::QuitCommand.execute(ctx).await,
            AppAction::ShowHelp => ui::ShowHelpCommand.execute(ctx).await,
            AppAction::Navigate(direction) => {
                navigation::NavigateCommand::new(direction.clone())
                    .execute(ctx)
                    .await
            }
            AppAction::SelectItem => navigation::SelectItemCommand.execute(ctx).await,
            AppAction::GoBack => ui::GoBackCommand.execute(ctx).await,
            AppAction::ShowTransitions => transition::ShowTransitionsCommand.execute(ctx).await,
            AppAction::ExecuteTransition(transition_id) => {
                transition::ExecuteTransitionCommand::new(transition_id.clone())
                    .execute(ctx)
                    .await
            }
            AppAction::None => Ok(CommandResult::Continue),
        }
    }
}

pub use crate::navigation::AppAction;
