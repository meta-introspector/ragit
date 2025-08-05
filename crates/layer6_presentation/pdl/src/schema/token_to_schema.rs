use super::schema_parse_error::SchemaParseError;
use super::token::{Token, GroupKind};
use ragit_types::schema::{Constraint, Schema, SchemaType};

pub fn token_to_schema(tokens: &[Token], index: &mut usize) -> Result<Schema, SchemaParseError> {
    let mut r#type = match tokens.get(*index) {
        Some(t @ Token::Literal(s)) => match s.as_str() {
            "str" | "string" => Schema::default_string(),
            "int" | "integer" => Schema::default_integer(),
            "float" | "number" => Schema::default_float(),
            "bool" | "boolean" => Schema::default_boolean(),
            "yesno" => Schema::default_yesno(),
            "code" => Schema::default_code(),
            "tasklist" => Schema::default_task_list(),
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

            Schema {
                r#type: SchemaType::Object(result),
                constraint: None,
            }
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

            Schema::default_array(inner_type)
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
        let constraint = super::parse_constraint::parse_constraint(inner)?;
        r#type.add_constraint(constraint);
        *index += 1;
    }

    Ok(r#type)
}
