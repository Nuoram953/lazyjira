#[cfg(test)]
mod tests {
    use crate::api::types::*;
    use serde_json::json;

    #[test]
    fn test_parse_string_description() {
        let json_data = json!({
            "summary": "Test issue",
            "description": "this is a simple string",
            "status": {"name": "Open"},
            "created": "2024-01-01T00:00:00.000Z",
            "updated": "2024-01-01T00:00:00.000Z",
            "issuetype": {"name": "Bug"}
        });

        let result: Result<JiraIssueFields, _> = serde_json::from_value(json_data);
        assert!(
            result.is_ok(),
            "Should parse string description successfully"
        );

        let fields = result.unwrap();
        assert!(fields.description.is_some());
        let desc = fields.description.unwrap();
        assert_eq!(desc.to_text(), Some("this is a simple string".to_string()));
    }

    #[test]
    fn test_parse_adf_description() {
        let json_data = json!({
            "summary": "Test issue",
            "description": {
                "type": "doc",
                "version": 1,
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": "This is ADF content"
                            }
                        ]
                    }
                ]
            },
            "status": {"name": "Open"},
            "created": "2024-01-01T00:00:00.000Z",
            "updated": "2024-01-01T00:00:00.000Z",
            "issuetype": {"name": "Bug"}
        });

        let result: Result<JiraIssueFields, _> = serde_json::from_value(json_data);
        assert!(result.is_ok(), "Should parse ADF description successfully");

        let fields = result.unwrap();
        assert!(fields.description.is_some());
        let desc = fields.description.unwrap();
        assert_eq!(desc.to_text(), Some("This is ADF content\n\n".to_string()));
        // ADF adds formatting newlines
    }

    #[test]
    fn test_parse_null_description() {
        let json_data = json!({
            "summary": "Test issue",
            "description": null,
            "status": {"name": "Open"},
            "created": "2024-01-01T00:00:00.000Z",
            "updated": "2024-01-01T00:00:00.000Z",
            "issuetype": {"name": "Bug"}
        });

        let result: Result<JiraIssueFields, _> = serde_json::from_value(json_data);
        assert!(result.is_ok(), "Should handle null description");

        let fields = result.unwrap();
        assert!(fields.description.is_none());
    }
}
