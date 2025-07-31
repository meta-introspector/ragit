pub enum ColumnAlignment {
    Left,
    Right,
    Center,
}

pub struct Column {
    pub header: String,
    pub width: usize,
    pub alignment: ColumnAlignment,
}

impl Column {
    pub fn new(header: String, width: usize) -> Self {
        Column {
            header,
            width,
            alignment: ColumnAlignment::Left, // Default to left alignment
        }
    }
}