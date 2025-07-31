enum NaturalLanguageParseState {
    Init,
    Integer,
    Float,
    Json(JsonParseState),
}

enum JsonParseState {
    Init,
    String { escape: bool },
}

#[derive(PartialEq)]
enum JsonGroup {
    Brace,
    Bracket,
}

impl From<u8> for JsonGroup {
    fn from(c: u8) -> JsonGroup {
        match c {
            b'{' | b'}' => JsonGroup::Brace,
            b'[' | b']' => JsonGroup::Bracket,
            _ => panic!(),
        }
    }
}

pub enum JsonMatch<'a> {
    NoMatch,
    MultipleMatches,
    Match(&'a str),
}
