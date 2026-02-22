use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AdfDocument {
    #[serde(rename = "type")]
    pub doc_type: String,
    pub version: u32,
    pub content: Option<Vec<AdfNode>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AdfNode {
    #[serde(rename = "paragraph")]
    Paragraph { content: Option<Vec<AdfNode>> },
    #[serde(rename = "text")]
    Text {
        text: String,
        marks: Option<Vec<AdfMark>>,
    },
    #[serde(rename = "hardBreak")]
    HardBreak,
    #[serde(rename = "heading")]
    Heading {
        attrs: Option<AdfHeadingAttrs>,
        content: Option<Vec<AdfNode>>,
    },
    #[serde(rename = "codeBlock")]
    CodeBlock {
        content: Option<Vec<AdfNode>>,
        attrs: Option<AdfCodeBlockAttrs>,
    },
    #[serde(rename = "bulletList")]
    BulletList { content: Option<Vec<AdfNode>> },
    #[serde(rename = "orderedList")]
    OrderedList { content: Option<Vec<AdfNode>> },
    #[serde(rename = "listItem")]
    ListItem { content: Option<Vec<AdfNode>> },
    #[serde(rename = "blockquote")]
    Blockquote { content: Option<Vec<AdfNode>> },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AdfMark {
    #[serde(rename = "strong")]
    Strong,
    #[serde(rename = "em")]
    Em,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "underline")]
    Underline,
    #[serde(rename = "strike")]
    Strike,
    #[serde(rename = "link")]
    Link { attrs: AdfLinkAttrs },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdfHeadingAttrs {
    pub level: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdfCodeBlockAttrs {
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AdfLinkAttrs {
    pub href: String,
}

impl AdfDocument {
    pub fn to_plain_text(&self) -> String {
        self.content
            .as_ref()
            .map(|nodes| Self::nodes_to_text(nodes))
            .unwrap_or_default()
    }

    pub fn to_formatted_text(&self) -> String {
        self.content
            .as_ref()
            .map(|nodes| Self::nodes_to_formatted_text(nodes, 0))
            .unwrap_or_default()
    }

    fn nodes_to_text(nodes: &[AdfNode]) -> String {
        nodes
            .iter()
            .map(Self::node_to_text)
            .collect::<Vec<_>>()
            .join("")
    }

    fn nodes_to_formatted_text(nodes: &[AdfNode], indent_level: usize) -> String {
        nodes
            .iter()
            .map(|node| Self::node_to_formatted_text(node, indent_level))
            .collect::<Vec<_>>()
            .join("")
    }

    fn node_to_text(node: &AdfNode) -> String {
        match node {
            AdfNode::Text { text, .. } => text.clone(),
            AdfNode::Paragraph { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!("{}\n", text)
            }
            AdfNode::HardBreak => "\n".to_string(),
            AdfNode::Heading { content, .. } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!("{}\n", text)
            }
            AdfNode::CodeBlock { content, .. } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!("{}\n", text)
            }
            AdfNode::BulletList { content } | AdfNode::OrderedList { content } => content
                .as_ref()
                .map(|nodes| Self::nodes_to_text(nodes))
                .unwrap_or_default(),
            AdfNode::ListItem { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!("• {}", text)
            }
            AdfNode::Blockquote { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!("> {}\n", text)
            }
            AdfNode::Unknown => String::new(),
        }
    }

    fn node_to_formatted_text(node: &AdfNode, indent_level: usize) -> String {
        let indent = "  ".repeat(indent_level);

        match node {
            AdfNode::Text { text, marks } => Self::apply_text_formatting(text, marks.as_ref()),
            AdfNode::Paragraph { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level))
                    .unwrap_or_default();
                format!("{}{}\n\n", indent, text.trim_end())
            }
            AdfNode::HardBreak => "\n".to_string(),
            AdfNode::Heading { content, attrs } => {
                let level = attrs.as_ref().map(|a| a.level).unwrap_or(1);
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level))
                    .unwrap_or_default();
                let prefix = "#".repeat(level as usize);
                format!("{}{} {}\n\n", indent, prefix, text.trim())
            }
            AdfNode::CodeBlock { content, .. } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_text(nodes))
                    .unwrap_or_default();
                format!(
                    "{}```\n{}{}\n{}```\n\n",
                    indent,
                    indent,
                    text.trim(),
                    indent
                )
            }
            AdfNode::BulletList { content } => content
                .as_ref()
                .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level))
                .unwrap_or_default(),
            AdfNode::OrderedList { content } => content
                .as_ref()
                .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level))
                .unwrap_or_default(),
            AdfNode::ListItem { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level + 1))
                    .unwrap_or_default();
                format!("{}• {}\n", indent, text.trim())
            }
            AdfNode::Blockquote { content } => {
                let text = content
                    .as_ref()
                    .map(|nodes| Self::nodes_to_formatted_text(nodes, indent_level))
                    .unwrap_or_default();
                format!("{}> {}\n", indent, text.trim())
            }
            AdfNode::Unknown => String::new(),
        }
    }

    fn apply_text_formatting(text: &str, marks: Option<&Vec<AdfMark>>) -> String {
        if let Some(marks) = marks {
            let mut result = text.to_string();

            for mark in marks {
                result = match mark {
                    AdfMark::Strong => format!("**{}**", result),
                    AdfMark::Em => format!("*{}*", result),
                    AdfMark::Code => format!("`{}`", result),
                    AdfMark::Underline => format!("_{}_", result),
                    AdfMark::Strike => format!("~~{}~~", result),
                    AdfMark::Link { attrs } => format!("[{}]({})", result, attrs.href),
                    AdfMark::Unknown => result,
                };
            }

            result
        } else {
            text.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_paragraph() {
        let json = r#"
        {
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "this is a test"
                        }
                    ]
                }
            ]
        }
        "#;

        let doc: AdfDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.to_plain_text().trim(), "this is a test");
    }

    #[test]
    fn test_parse_with_formatting() {
        let json = r#"
        {
            "type": "doc",
            "version": 1,
            "content": [
                {
                    "type": "paragraph",
                    "content": [
                        {
                            "type": "text",
                            "text": "bold text",
                            "marks": [{"type": "strong"}]
                        }
                    ]
                }
            ]
        }
        "#;

        let doc: AdfDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.to_formatted_text().trim(), "**bold text**");
    }
}
