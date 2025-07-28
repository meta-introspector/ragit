use serde::{Deserialize, Serialize};
use std::fmt;

use crate::fixed_types::fixed_string::FixedString;
use crate::fixed_types::fixed_vec::FixedVec;

// Define constants for FixedHashMap sizes
const FIXED_HASH_MAP_KEY_SIZE: usize = 128;
const FIXED_HASH_MAP_VALUE_SIZE: usize = 128;
const FIXED_HASH_MAP_CAPACITY: usize = 8;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedHashMap<const K: usize, const V: usize, const N: usize> {
    data: FixedVec<(FixedString<K>, FixedString<V>), N>,
}

impl<const K: usize, const V: usize, const N: usize> FixedHashMap<K, V, N> {
    pub fn new() -> Self {
        Self {
            data: FixedVec::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        // Check if key already exists, update if so
        for (existing_key, existing_value) in self.data.iter_mut() {
            if existing_key.as_str() == key {
                *existing_value = value.into();
                return;
            }
        }
        // If key does not exist and there's capacity, insert new
        if self.data.len() < N {
            self.data.push((key.into(), value.into()));
        } else {
            eprintln!("FixedHashMap overflow: capacity {} reached.", N);
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        for (existing_key, existing_value) in self.data.iter() {
            if existing_key.as_str() == key {
                return Some(existing_value.as_str());
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.data.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

impl<const K: usize, const V: usize, const N: usize> Default for FixedHashMap<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const K: usize, const V: usize, const N: usize> From<std::collections::HashMap<String, String>> for FixedHashMap<K, V, N> {
    fn from(hash_map: std::collections::HashMap<String, String>) -> Self {
        let mut fhm = Self::new();
        for (key, value) in hash_map {
            fhm.insert(&key, &value);
        }
        fhm
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_fixed_hash_map_new() {
        let fhm: FixedHashMap<10, 10, 5> = FixedHashMap::new();
        assert_eq!(fhm.len(), 0);
        assert!(fhm.is_empty());
    }

    #[test]
    fn test_fixed_hash_map_insert_and_get() {
        let mut fhm: FixedHashMap<10, 10, 5> = FixedHashMap::new();
        fhm.insert("key1", "value1");
        fhm.insert("key2", "value2");
        assert_eq!(fhm.len(), 2);
        assert_eq!(fhm.get("key1"), Some("value1"));
        assert_eq!(fhm.get("key2"), Some("value2"));
        assert_eq!(fhm.get("key3"), None);
    }

    #[test]
    fn test_fixed_hash_map_update() {
        let mut fhm: FixedHashMap<10, 10, 5> = FixedHashMap::new();
        fhm.insert("key1", "value1");
        fhm.insert("key1", "new_value");
        assert_eq!(fhm.len(), 1);
        assert_eq!(fhm.get("key1"), Some("new_value"));
    }

    #[test]
    fn test_fixed_hash_map_overflow() {
        let mut fhm: FixedHashMap<10, 10, 2> = FixedHashMap::new();
        fhm.insert("k1", "v1");
        fhm.insert("k2", "v2");
        assert_eq!(fhm.len(), 2);
        fhm.insert("k3", "v3"); // Should overflow
        assert_eq!(fhm.len(), 2);
        assert_eq!(fhm.get("k3"), None);
    }

    #[test]
    fn test_fixed_hash_map_from_std_hash_map() {
        let mut std_map = HashMap::new();
        std_map.insert(String::from("k1"), String::from("v1"));
        std_map.insert(String::from("k2"), String::from("v2"));

        let fhm: FixedHashMap<10, 10, 5> = std_map.into();
        assert_eq!(fhm.len(), 2);
        assert_eq!(fhm.get("k1"), Some("v1"));
        assert_eq!(fhm.get("k2"), Some("v2"));
    }

    #[test]
    fn test_fixed_hash_map_iter() {
        let mut fhm: FixedHashMap<10, 10, 5> = FixedHashMap::new();
        fhm.insert("a", "1");
        fhm.insert("b", "2");
        let mut items: Vec<(&str, &str)> = fhm.iter().collect();
        items.sort_unstable(); // Sort for consistent testing
        assert_eq!(items, vec![("a", "1"), ("b", "2")]);
    }
}
