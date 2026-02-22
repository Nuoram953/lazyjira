use super::{Command, CommandResult};
use crate::context::AppContext;
use crate::navigation::AppView;
use async_trait::async_trait;
use color_eyre::Result;

pub struct QuitCommand;

#[async_trait]
impl Command for QuitCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        ctx.should_quit = true;
        Ok(CommandResult::Quit)
    }

    fn description(&self) -> &str {
        "Quit application"
    }
}

pub struct ShowHelpCommand;

#[async_trait]
impl Command for ShowHelpCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        ctx.navigation.current_view = AppView::Help;
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        "Show help"
    }
}

pub struct GoBackCommand;

#[async_trait]
impl Command for GoBackCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        use crate::navigation::FocusedPane;

        match ctx.navigation.current_view {
            AppView::Help => {
                ctx.navigation.current_view = AppView::Main;
            }
            AppView::TransitionSelector => {
                ctx.navigation.hide_transitions();
                ctx.available_transitions.clear();
                ctx.transitioning_issue_key = None;
            }
            AppView::Main => {
                if ctx.navigation.focused_pane == FocusedPane::Detail {
                    ctx.navigation.go_back_from_detail();
                }
            }
        }
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        "Go back to previous view"
    }
}
