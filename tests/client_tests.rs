use base64::prelude::*;
use lazyjira::services::client::*;
use std::env;

#[test]
fn test_build_auth_header() {
    let username = "test@example.com";
    let api_token = "secret_token";
    let result = JiraClient::build_auth_header(username, api_token);

    let expected_credentials = "test@example.com:secret_token";
    let expected_encoded = BASE64_STANDARD.encode(expected_credentials.as_bytes());
    let expected = format!("Basic {}", expected_encoded);

    assert_eq!(result, expected);
}

#[test]
fn test_build_auth_header_with_special_chars() {
    let username = "user+test@domain.com";
    let api_token = "token!@#$%^&*()";
    let result = JiraClient::build_auth_header(username, api_token);

    assert!(result.starts_with("Basic "));
    assert!(result.len() > 6); // More than just "Basic "
}

#[test]
fn test_build_auth_header_empty_values() {
    let result = JiraClient::build_auth_header("", "");
    let expected_encoded = BASE64_STANDARD.encode(":".as_bytes());
    let expected = format!("Basic {}", expected_encoded);

    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_new_client_with_default_config() {
    // Store original environment variables
    let original_base_url = env::var("JIRA_BASE_URL").ok();
    let original_username = env::var("JIRA_USERNAME").ok();
    let original_api_token = env::var("JIRA_API_TOKEN").ok();
    let original_board_id = env::var("JIRA_BOARD_ID").ok();

    // Clear environment variables to test default behavior
    env::remove_var("JIRA_BASE_URL");
    env::remove_var("JIRA_USERNAME");
    env::remove_var("JIRA_API_TOKEN");
    env::remove_var("JIRA_BOARD_ID");

    // Verify variables are actually cleared
    assert!(env::var("JIRA_BASE_URL").is_err());
    assert!(env::var("JIRA_USERNAME").is_err());
    assert!(env::var("JIRA_API_TOKEN").is_err());
    assert!(env::var("JIRA_BOARD_ID").is_err());

    // This should succeed because JiraConfig::from_env() provides defaults
    let result = JiraClient::new();
    assert!(
        result.is_ok(),
        "Client creation should succeed with default config"
    );

    let client = result.unwrap();
    // Verify it uses default values
    assert_eq!(client.config.base_url, "https://your-domain.atlassian.net");
    assert_eq!(client.config.username, "your-email@example.com");
    assert_eq!(client.config.api_token, "your-api-token");
    assert_eq!(client.config.board_id, "1");

    // Restore original environment variables if they were set
    if let Some(val) = original_base_url {
        env::set_var("JIRA_BASE_URL", val);
    }
    if let Some(val) = original_username {
        env::set_var("JIRA_USERNAME", val);
    }
    if let Some(val) = original_api_token {
        env::set_var("JIRA_API_TOKEN", val);
    }
    if let Some(val) = original_board_id {
        env::set_var("JIRA_BOARD_ID", val);
    }
}

#[test]
fn test_sprint_issues_query_serialization() {
    let query = SprintIssuesQuery {
        jql: "project = TEST".to_string(),
        max_results: Some(25),
        start_at: Some(0),
    };

    let serialized = serde_json::to_value(&query).unwrap();
    assert_eq!(serialized["jql"], "project = TEST");
    assert_eq!(serialized["maxResults"], 25);
    assert_eq!(serialized["startAt"], 0);
}

#[test]
fn test_sprint_issues_query_optional_fields() {
    let query = SprintIssuesQuery {
        jql: "project = TEST".to_string(),
        max_results: None,
        start_at: None,
    };

    let serialized = serde_json::to_value(&query).unwrap();
    assert_eq!(serialized["jql"], "project = TEST");
    assert!(serialized["maxResults"].is_null());
    assert!(serialized["startAt"].is_null());
}
