use serde_json::Value;

pub fn extract_text_from_adf(content: &Value) -> String {
    let mut text = String::new();

    if let Some(content_array) = content.as_array() {
        for item in content_array {
            let item_type = item.get("type").and_then(|t| t.as_str());

            let mut paragraph_text = String::new();

            if let Some(content_inner) = item.get("content") {
                if let Some(inner_array) = content_inner.as_array() {
                    for inner_item in inner_array {
                        if let Some(text_value) = inner_item.get("text") {
                            if let Some(text_str) = text_value.as_str() {
                                paragraph_text.push_str(text_str);
                            }
                        }

                        if let Some(nested_content) = inner_item.get("content") {
                            paragraph_text.push_str(&extract_text_from_adf(nested_content));
                        }
                    }
                }
            }

            if !paragraph_text.trim().is_empty() {
                text.push_str(paragraph_text.trim());

                match item_type {
                    Some("paragraph") => {
                        text.push('\n');
                        text.push('\n')
                    }
                    Some("heading") => text.push('\n'),
                    Some("bulletList") | Some("orderedList") => text.push('\n'),
                    Some("listItem") => text.push('\n'),
                    Some("codeBlock") => text.push('\n'),
                    _ => {
                        if !text.ends_with('\n') && !text.ends_with(' ') {
                            text.push(' ');
                        }
                    }
                }
            }
        }
    }

    text.trim_end().to_string()
}

pub fn parse_description(description: Option<&Value>) -> String {
    match description {
        Some(desc_value) => {
            if let Some(desc_str) = desc_value.as_str() {
                desc_str.to_string()
            } else if let Some(content_array) = desc_value.get("content") {
                extract_text_from_adf(content_array)
            } else {
                desc_value.to_string()
            }
        }
        None => "No description available".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_simple_paragraph() {
        let adf_content = json!([
            {
                "type": "paragraph",
                "content": [
                    {
                        "type": "text",
                        "text": "This is a simple paragraph."
                    }
                ]
            }
        ]);

        let result = extract_text_from_adf(&adf_content);
        assert_eq!(result, "This is a simple paragraph.");
    }

    #[test]
    fn test_extract_multiple_paragraphs() {
        let adf_content = json!([
            {
                "type": "paragraph",
                "content": [
                    {
                        "type": "text",
                        "text": "First paragraph."
                    }
                ]
            },
            {
                "type": "paragraph",
                "content": [
                    {
                        "type": "text",
                        "text": "Second paragraph."
                    }
                ]
            }
        ]);

        let result = extract_text_from_adf(&adf_content);
        assert_eq!(result, "First paragraph.\nSecond paragraph.");
    }

    #[test]
    fn test_parse_description_string() {
        let desc = json!("Simple string description");
        let result = parse_description(Some(&desc));
        assert_eq!(result, "Simple string description");
    }

    #[test]
    fn test_parse_description_adf() {
        let desc = json!({
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "ADF description."
                        }
                    ]
                }
            ]
        });

        let result = parse_description(Some(&desc));
        assert_eq!(result, "ADF description.");
    }

    #[test]
    fn test_parse_description_none() {
        let result = parse_description(None);
        assert_eq!(result, "No description available");
    }
}
