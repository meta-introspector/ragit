use crate::{CodeSnippet, TestResult};

pub fn test_code_snippet(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    match snippet.language.as_str() {
        "rust" => {
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        "javascript" | "js" => {
            if snippet.content.contains("function") || snippet.content.contains("const ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        _ => {
            snippet.test_result = Some(TestResult {
                success: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }
    }
}
