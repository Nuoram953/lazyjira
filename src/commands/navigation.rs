use super::{Command, CommandResult};
use crate::context::AppContext;
use crate::navigation::{Direction, FocusedPane};
use async_trait::async_trait;
use color_eyre::Result;

pub struct NavigateCommand {
    direction: Direction,
}

impl NavigateCommand {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }
}

#[async_trait]
impl Command for NavigateCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        match self.direction {
            Direction::Left | Direction::Right => {
                ctx.navigation.move_focus(self.direction.clone());
            }
            Direction::Up | Direction::Down => {
                if ctx.navigation.focused_pane == FocusedPane::Detail {
                    ctx.navigation.move_focus(self.direction.clone());
                } else {
                    let max_items = match ctx.navigation.focused_pane {
                        FocusedPane::Sprint => {
                            if let Some(sprint) = &ctx.data.current_sprint {
                                sprint.issues.len()
                            } else {
                                0
                            }
                        }
                        FocusedPane::Board => ctx.data.board_issues.len(),
                        FocusedPane::LastUpdated => ctx.data.last_updated_issues.len(),
                        FocusedPane::Detail => 0,
                        FocusedPane::TransitionList => ctx.available_transitions.len(),
                    };

                    ctx.navigation
                        .move_selection(self.direction.clone(), max_items);
                    update_selected_issue(ctx);
                }
            }
        }
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        match self.direction {
            Direction::Up => "Navigate up",
            Direction::Down => "Navigate down",
            Direction::Left => "Navigate left",
            Direction::Right => "Navigate right",
        }
    }
}

pub struct SelectItemCommand;

#[async_trait]
impl Command for SelectItemCommand {
    async fn execute(&self, ctx: &mut AppContext) -> Result<CommandResult> {
        match ctx.navigation.focused_pane {
            FocusedPane::Sprint | FocusedPane::Board | FocusedPane::LastUpdated => {
                update_selected_issue(ctx);
                ctx.navigation.focus_detail();
            }
            FocusedPane::Detail => {}
            FocusedPane::TransitionList => {}
        }
        Ok(CommandResult::Continue)
    }

    fn description(&self) -> &str {
        "Select current item"
    }
}

fn update_selected_issue(ctx: &mut AppContext) {
    match ctx.navigation.focused_pane {
        FocusedPane::Sprint => {
            if let Some(sprint) = &ctx.data.current_sprint {
                if let Some(issue) = sprint.issues.get(ctx.navigation.sprint_selected) {
                    ctx.data.selected_issue = Some(issue.clone());
                }
            }
        }
        FocusedPane::Board => {
            if let Some(issue) = ctx
                .data
                .board_issues
                .get(ctx.navigation.last_viewed_selected)
            {
                ctx.data.selected_issue = Some(issue.clone());
            }
        }
        FocusedPane::LastUpdated => {
            if let Some(issue) = ctx
                .data
                .last_updated_issues
                .get(ctx.navigation.last_updated_selected)
            {
                ctx.data.selected_issue = Some(issue.clone());
            }
        }
        FocusedPane::Detail => {}
        FocusedPane::TransitionList => {}
    }
}
