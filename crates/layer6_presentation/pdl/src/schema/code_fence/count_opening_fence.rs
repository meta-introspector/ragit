pub fn count_opening_fence(line: &str) -> Option<usize> {
    let mut backtick_count = 0;
    let mut no_more_backticks = false;

    for c in line.chars() {
        match c {
            '`' => {
                if no_more_backticks {
                    return None;
                } else {
                    backtick_count += 1;
                }
            }
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                if backtick_count < 3 {
                    return None;
                }

                no_more_backticks = true;
            }
            ' ' => match backtick_count {
                0 => {}
                1..=2 => {
                    return None;
                }
                3.. => {
                    no_more_backticks = true;
                }
            },
            _ => {
                return None;
            }
        }
    }

    if backtick_count >= 3 {
        Some(backtick_count)
    } else {
        None
    }
}
