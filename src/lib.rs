//! LazyJira - A Terminal User Interface for Jira
//!
//! This library provides the core functionality for the LazyJira TUI application,
//! including Jira API client, data models, navigation logic, and UI components.

pub mod app;
pub mod core;
pub mod panes;
pub mod services;
pub mod ui;

// Re-export commonly used types and modules for easier access in tests
pub use app::{messages::AppMessage, navigator::Navigator, state::App};
pub use services::{client::JiraClient, config::JiraConfig, types::*};
pub use ui::components::issue_list::IssueList;
