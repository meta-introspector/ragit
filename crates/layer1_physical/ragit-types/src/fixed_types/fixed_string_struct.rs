use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedString<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    pub fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    pub fn as_str(&self) -> &str {
        // This assumes valid UTF-8. In a real scenario, you might want to handle errors.
        std::str::from_utf8(&self.data[..self.len]).expect("Invalid UTF-8 in FixedString")
    }

    pub fn push_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let bytes_to_copy = std::cmp::min(bytes.len(), N - self.len);
        self.data[self.len..self.len + bytes_to_copy].copy_from_slice(&bytes[..bytes_to_copy]);
        self.len += bytes_to_copy;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
