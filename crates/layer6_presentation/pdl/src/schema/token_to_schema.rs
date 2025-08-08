use super::schema_parse_error::SchemaParseError;
use super::token::{Token, GroupKind};
use ragit_types::schema::{Schema, default_string, default_integer, default_float, default_boolean, default_yesno, default_code, default_task_list, default_array};

pub fn token_to_schema(tokens: &[Token], index: &mut usize) -> Result<Schema, SchemaParseError> {
    let r#type = match tokens.get(*index) {
        Some(t @ Token::Literal(s)) => match s.as_str() {
            "str" | "string" => default_string(),
            "int" | "integer" => default_integer(),
            "float" | "number" => default_float(),
            "bool" | "boolean" => default_boolean(),
            "yesno" => default_yesno(),
            "code" => default_code(),
            "tasklist" => default_task_list(),
            _ => {
                return Err(SchemaParseError::UnexpectedToken(t.clone()));
            }
        },
        Some(Token::Group {
            kind: GroupKind::Brace,
            tokens: inner,
        }) => {
            let mut inner_index = 0;
            let mut result = vec![];

            loop {
                let key = match inner.get(inner_index) {
                    Some(Token::Literal(s)) => s.to_string(),
                    Some(t) => {
                        return Err(SchemaParseError::UnexpectedToken(t.clone()));
                    }
                    None => {
                        break;
                    }
                };

                inner_index += 1;

                match inner.get(inner_index) {
                    Some(Token::Punct(b':')) => {}
                    Some(t) => {
                        return Err(SchemaParseError::UnexpectedToken(t.clone()));
                    }
                    None => {
                        break;
                    }
                }

                inner_index += 1;
                let inner_type = token_to_schema(inner, &mut inner_index)?;
                result.push((key, inner_type));

                match inner.get(inner_index) {
                    Some(Token::Punct(b',')) => {
                        inner_index += 1;
                    }
                    Some(t) => {
                        return Err(SchemaParseError::UnexpectedToken(t.clone()));
                    }
                    None => {
                        break;
                    }
                }
            }

            Schema(serde_json::json!({
                "type": "object",
                "properties": result.into_iter().map(|(k, v)| {
                    (k, serde_json::from_str::<serde_json::Value>(&v.0).unwrap())
                }).collect::<std::collections::HashMap<String, serde_json::Value>>()
            }).to_string())
        }
        Some(Token::Group {
            kind: GroupKind::Bracket,
            tokens: inner,
        }) => {
            let mut inner_index = 0;
            let inner_type = if inner.is_empty() {
                None
            } else {
                let res = token_to_schema(inner, &mut inner_index)?;

                if inner_index < inner.len() {
                    return Err(SchemaParseError::UnexpectedToken(
                        inner[inner_index].clone(),
                    ));
                }

                Some(res)
            };

                            default_array(inner_type.expect("Expected inner_type to be Some"))
        }
        Some(t) => {
            return Err(SchemaParseError::UnexpectedToken(t.clone()));
        }
        None => {
            return Err(SchemaParseError::UnexpectedEof);
        }
    };
    *index += 1;

    if let Some(Token::Group {
        kind: GroupKind::Brace,
        tokens: inner,
    }) = tokens.get(*index)
    {
        let _constraint = super::parse_constraint::parse_constraint(inner)?;
        
        *index += 1;
    }

    Ok(r#type)
}
