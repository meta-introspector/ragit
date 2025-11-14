use super::parse_state::ParseState;

pub fn try_extract_code_fence(s: &str) -> Result<String, String> {
    let mut state = ParseState::Before;
    let mut code_lines = vec![];

    for line in s.lines() {
        match state {
            ParseState::Before => {
                if let Some(n) = super::count_opening_fence::count_opening_fence(line) {
                    state = ParseState::In(n);
                }
            }
            ParseState::In(fence_len) => {
                if let Some(n) = super::count_closing_fence::count_closing_fence(line) {
                    if n >= fence_len {
                        state = ParseState::After;
                    } else {
                        code_lines.push(line.to_string());
                    }
                }
            } else {
                code_lines.push(line.to_string());
            }
        }
        ParseState::After => {
            if super::count_opening_fence::count_opening_fence(line).is_some() {
                return Err(String::from(
                    "It seems like your response has more than 1 code block. Please give me exactly 1 code block.",
                ));
            }
        }
    }

    if let ParseState::Before = state {
        Err(String::from(
            "I cannot find a code block in your response. Please give me a fenced code block. An opening and closing fence consist of 3 or more backtick characters.",
        ))
    }
    // [spec](https://github.github.com/gfm/#fenced-code-blocks) allows omitting a closing fence
    else {
        let result = code_lines.join("\n");
        Ok(result.trim().to_string())
    }
}
