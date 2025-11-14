
use ragit_types::pdl_error::PdlError as Error;
use crate::pdl_struct::Pdl;
use ragit_types::schema::Schema;

use ragit_types::pdl_types::{Message, MessageContent, PdlRole};
use crate::escape_pdl_tokens::escape_pdl_tokens;
use crate::into_message_contents::into_message_contents;

pub fn parse_pdl_logic(
    s: &str,
    context: &tera::Context,
    curr_dir: &str,
    strict_mode: bool,
) -> Result<Pdl, Error> {
    let mut renderer = tera::Tera::default();
    renderer.autoescape_on(vec!["__tera_one_off"]);
    renderer.set_escape_fn(escape_pdl_tokens);

    let tera_rendered = match renderer.render_str(s, context) {
        Ok(t) => t,
        Err(e) => {
            if strict_mode {
                return Err(e.into());
            } else {
                s.to_string()
            }
        }
    };

    let mut messages = vec![];
    let mut schema = None;
    let mut curr_role = None;
    let mut line_buffer = vec![];

    let last_line = "<|assistant|>";

    for line in tera_rendered.lines().chain(std::iter::once(last_line)) {
        let trimmed = line.trim();

        if trimmed.starts_with("<|") && trimmed.ends_with("|>") && trimmed.len() > 4 {
            match trimmed
                .to_ascii_lowercase()
                .get(2..(trimmed.len() - 2))
                .unwrap()
            {
                t @ ("user" | "system" | "assistant" | "schema" | "reasoning") => {
                    if !line_buffer.is_empty() || curr_role.is_some() {
                        match curr_role {
                            Some(PdlRole::Schema) => {
                                if schema.is_some() && strict_mode {
                                    return Err(Error::InvalidPdl(String::from(
                                        "<|schema|> appeared multiple times.",
                                    )));
                                }
                                schema = Some(Schema(line_buffer.join("\n")));
                            }
                            Some(PdlRole::Reasoning) => {} // reasoning tokens are not fed to llm contexts
                            _ => {
                                let raw_contents = line_buffer.join("\n");
                                let raw_contents = raw_contents.trim();

                                let role = match curr_role {
                                    Some(role) => role,
                                    None => {
                                        if raw_contents.is_empty() {
                                            curr_role = Some(PdlRole::from(t));
                                            line_buffer = vec![];
                                            continue;
                                        }

                                        if strict_mode {
                                            return Err(Error::RoleMissing);
                                        } else {
                                            // TODO: Handle non-strict mode if necessary
                                        }
                                        PdlRole::System
                                    }
                                };

                                match into_message_contents(raw_contents, curr_dir) {
                                    Ok(t) => {
                                        messages.push(Message {
                                            role: role.into(),
                                            content: t,
                                        });
                                    }
                                    Err(e) => {
                                        if strict_mode {
                                            return Err(e);
                                        } else {
                                            messages.push(Message {
                                                role: role.into(),
                                                content: vec![MessageContent::String(
                                                    raw_contents.to_string(),
                                                )],
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    curr_role = Some(PdlRole::from(t));
                    line_buffer = vec![];
                    continue;
                }
                t => {
                    if strict_mode && t.chars().all(|c| c.is_ascii_alphabetic()) {
                        return Err(Error::InvalidTurnSeparator(t.to_string()));
                    }

                    line_buffer.push(line.to_string());
                }
            }
        } else {
            line_buffer.push(line.to_string());
        }
    }

    if let Some(Message { content, .. }) = messages.last() {
        if content.is_empty() {
            messages.pop().unwrap();
        }
    }

    let result = Pdl { schema, messages };

    if strict_mode {
        result.validate()?;
    }

    Ok(result)
}
