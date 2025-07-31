pub fn try_get_pdl_token(bytes: &[u8], mut index: usize) -> Option<(&[u8], usize)> {
    let old_index = index;

    match (bytes.get(index), bytes.get(index + 1)) {
        (Some(b'<'), Some(b'|')) => {
            index += 2;

            loop {
                match (bytes.get(index), bytes.get(index + 1)) {
                    (Some(b'|'), Some(b'>')) => {
                        return Some((&bytes[(old_index + 2)..index], index + 2));
                    }
                    (_, Some(b'|')) => {
                        index += 1;
                    }
                    (_, None) => {
                        return None;
                    }
                    _ => {
                        index += 2;
                    }
                }
            }
        }
        _ => None,
    }
}
