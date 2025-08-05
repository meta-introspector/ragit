use ragit_types::schema::{Constraint};
use super::schema_parse_error::SchemaParseError;
use super::token::Token;

pub fn parse_constraint(tokens: &[Token]) -> Result<Constraint, SchemaParseError> {
    let mut index = 0;
    let mut result = Constraint::MinLength(0); // Placeholder, will be replaced by actual parsing

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
                Some(Token::Integer(n)) => {
                    if k == "min" || k == "len_min" {
                        if let Constraint::Minimum(min_val) = result {
                            return Err(SchemaParseError::InvalidConstraint(format!("Duplicate minimum constraint")));
                        }

                        result = Constraint::Minimum(*n as i128);
                    } else {
                        if let Constraint::Maximum(max_val) = result {
                            return Err(SchemaParseError::InvalidConstraint(format!("Duplicate maximum constraint")));
                        }

                        result = Constraint::Maximum(*n as i128);
                    }
                }
                Some(Token::Float(n)) => {
                    if k == "min" || k == "len_min" {
                        if let Constraint::Minimum(min_val) = result {
                            return Err(SchemaParseError::InvalidConstraint(format!("Duplicate minimum constraint")));
                        }

                        result = Constraint::Minimum(*n as i128);
                    } else {
                        if let Constraint::Maximum(max_val) = result {
                            return Err(SchemaParseError::InvalidConstraint(format!("Duplicate maximum constraint")));
                        }

                        result = Constraint::Maximum(*n as i128);
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
