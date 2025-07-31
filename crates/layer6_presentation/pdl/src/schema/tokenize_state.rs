pub enum TokenizeState {
    Init,
    Number,
    Identifier,
    Literal(u8),
}
