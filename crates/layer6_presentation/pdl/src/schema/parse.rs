use ragit_types::schema::{Schema};
use super::token::{Token};
use super::schema_parse_error::SchemaParseError;
use super::tokenize_state::TokenizeState;

pub fn parse_schema(s: &str) -> Result<Schema, SchemaParseError> {
    let mut index = 0;
    let s = s.as_bytes();
    let tokens = super::tokenize::tokenize(s, &mut index)?;

    if let Some(b) = s.get(index) {
        return Err(SchemaParseError::UnexpectedByte(*b));
    }

    let mut index = 0;
    let result = super::token_to_schema::token_to_schema(&tokens, &mut index)?;
    

    Ok(result)
}