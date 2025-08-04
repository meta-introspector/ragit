use markdown::{to_mdast, ParseOptions};
use markdown::mdast::{Node, Code, InlineCode};
use crate::{CodeSnippet, create_content_hash, estimate_token_count};

pub fn extract_code_snippets(text: &str) -> Result<Vec<CodeSnippet>, markdown::message::Message> {
    let mut snippets = Vec::new();
    let ast = to_mdast(text, &ParseOptions::default())?;
    walk_ast(&ast, &mut snippets);
    Ok(snippets)
}

pub fn walk_ast(node: &Node, snippets: &mut Vec<CodeSnippet>) {
    match node {
        Node::Code(code) => {
            let trimmed_content = code.value.trim();
            if !trimmed_content.is_empty() {
                let content_hash = create_content_hash(trimmed_content);
                let token_count = estimate_token_count(trimmed_content);
                let line_count = trimmed_content.lines().count();
                let char_count = trimmed_content.chars().count();
                let language = code.lang.as_deref().unwrap_or("text").to_string();

                snippets.push(CodeSnippet {
                    content: trimmed_content.to_string(),
                    content_hash,
                    language,
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                });
            }
        }
        Node::InlineCode(inline_code) => {
            let trimmed_content = inline_code.value.trim();
            if trimmed_content.len() >= 5 {
                let content_hash = create_content_hash(trimmed_content);
                let token_count = estimate_token_count(trimmed_content);
                let line_count = trimmed_content.lines().count();
                let char_count = trimmed_content.chars().count();

                snippets.push(CodeSnippet {
                    content: trimmed_content.to_string(),
                    content_hash,
                    language: "inline".to_string(),
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                });
            }
        }
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    walk_ast(child, snippets);
                }
            }
        }
    }
}
