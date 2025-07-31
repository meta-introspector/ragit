use super::schema_parse_error::SchemaParseError;
use super::token::Token;
use super::super::{Constraint, Schema};

pub fn parse_constraint(tokens: &[Token]) -> Result<Constraint, SchemaParseError> {
    let mut index = 0;
    let mut result = Constraint::default();

    loop {
        let key = match tokens.get(index) {
            Some(Token::Literal(s)) => s.to_string(),
            Some(t) => {
                return Err(SchemaParseError::UnexpectedToken(t.clone()));
            }
            None => {
                break;
            }
        };
        index += 1;

        match tokens.get(index) {
            Some(Token::Punct(b':')) => {}
            Some(t) => {
                return Err(SchemaParseError::UnexpectedToken(t.clone()));
            }
            None => {
                return Err(SchemaParseError::UnexpectedEof);
            }
        }

        index += 1;

        match key.as_str() {
            k @ ("min" | "max" | "len_min" | "len_max") => match tokens.get(index) {
                Some(n @ (Token::Integer(_) | Token::Float(_))) => {
                    if k == "min" || k == "len_min" {
                        if result.min.is_some() {
                            return Err(SchemaParseError::InvalidConstraint(format!(
                                "A constraint `{key}` appears more than once."
                            )));
                        }

                        result.min = Some(n.to_string());
                    } else {
                        if result.max.is_some() {
                            return Err(SchemaParseError::InvalidConstraint(format!(
                                "A constraint `{key}` appears more than once."
                            )));
                        }

                        result.max = Some(n.to_string());
                    }
                }
                Some(t) => {
                    return Err(SchemaParseError::UnexpectedToken(t.clone()));
                }
                None => {
                    return Err(SchemaParseError::UnexpectedEof);
                }
            },
            _ => {
                return Err(SchemaParseError::InvalidConstraint(format!(
                    "`{key}` is not a valid constraint"
                )));
            }
        }

        index += 1;

        match tokens.get(index) {
            Some(Token::Punct(b',')) => {}
            Some(t) => {
                return Err(SchemaParseError::UnexpectedToken(t.clone()));
            }
            None => {
                return Ok(result);
            }
        }

        index += 1;
    }

    Ok(result)
}
