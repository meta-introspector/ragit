use super::SchemaType;

impl SchemaType {
    pub fn is_number(&self) -> bool {
        matches!(self, SchemaType::Integer | SchemaType::Float)
    }
}
