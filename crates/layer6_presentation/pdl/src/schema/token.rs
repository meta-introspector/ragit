use std::fmt;

#[derive(Clone, Debug)]
pub enum Token {
    Literal(String),
    Integer(i64),
    Float(f64),
    Group {
        kind: GroupKind,
        tokens: Vec<Token>,
    },

    /// ':' | ','
    Punct(u8),
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Token::Literal(s) => write!(fmt, "{s:?}"),
            Token::Integer(n) => write!(fmt, "{n:?}"),
            Token::Float(n) => write!(fmt, "{n:?}"),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GroupKind {
    Brace,
    Parenthesis,
    Bracket,
}

impl From<u8> for GroupKind {
    fn from(c: u8) -> Self {
        match c {
            b'{' | b'}' => GroupKind::Brace,
            b'(' | b')' => GroupKind::Parenthesis,
            b'[' | b']' => GroupKind::Bracket,
            _ => unreachable!(),
        }
    }
}
