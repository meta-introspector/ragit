/// Defines a generative rule (lambda) for a specific type.
pub fn the_spark(i: usize) -> char {
    (i as u8 % 26 + b'a') as char
}
