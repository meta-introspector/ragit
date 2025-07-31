use super::fixed_string_struct::FixedString;
use std::fmt;

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

#[cfg(test)]
mod tests {
    
    use crate::fixed_types::fixed_string_struct::FixedString;

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
