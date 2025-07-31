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

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> From<&str> for FixedString<N> {
    fn from(s: &str) -> Self {
        let mut fs = Self::new();
        fs.push_str(s);
        fs
    }
}

impl<const N: usize> From<String> for FixedString<N> {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl<const N: usize> fmt::Display for FixedString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> Serialize for FixedString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FixedStringVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for FixedStringVisitor<N> {
            type Value = FixedString<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FixedString::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(FixedString::from(value))
            }
        }

        deserializer.deserialize_string(FixedStringVisitor::<N>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_string_new() {
        let fs: FixedString<10> = FixedString::new();
        assert_eq!(fs.len(), 0);
        assert_eq!(fs.as_str(), "");
    }

    #[test]
    fn test_fixed_string_from_str() {
        let fs: FixedString<10> = "hello".into();
        assert_eq!(fs.len(), 5);
        assert_eq!(fs.as_str(), "hello");
    }

    #[test]
    fn test_fixed_string_from_string() {
        let s = String::from("world");
        let fs: FixedString<10> = s.into();
        assert_eq!(fs.len(), 5);
        assert_eq!(fs.as_str(), "world");
    }

    #[test]
    fn test_fixed_string_truncation() {
        let fs: FixedString<5> = "hello world".into();
        assert_eq!(fs.len(), 5);
        assert_eq!(fs.as_str(), "hello");
    }

    #[test]
    fn test_fixed_string_push_str() {
        let mut fs: FixedString<10> = FixedString::new();
        fs.push_str("abc");
        assert_eq!(fs.len(), 3);
        assert_eq!(fs.as_str(), "abc");
        fs.push_str("defghijk"); // "abcdefghij"
        assert_eq!(fs.len(), 10);
        assert_eq!(fs.as_str(), "abcdefghij");
        fs.push_str("lmn"); // No more space
        assert_eq!(fs.len(), 10);
        assert_eq!(fs.as_str(), "abcdefghij");
    }

    #[test]
    fn test_fixed_string_empty() {
        let fs: FixedString<10> = FixedString::new();
        assert!(fs.is_empty());
        let fs2: FixedString<10> = "a".into();
        assert!(!fs2.is_empty());
    }
}
