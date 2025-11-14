pub fn matching_delim(c: u8) -> u8 {
    match c {
        b'{' => b'}',
        b'(' => b')',
        b'[' => b']',
        b'}' => b'{',
        b')' => b'(',
        b']' => b'[',
        _ => unreachable!(),
    }
}
