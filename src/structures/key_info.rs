use crate::add_thousands_separator;
use serde::{Deserialize, Serialize};

/// This key will be used by FileInfo and GroupInfo.
///
/// For the FileInfo struct, the hash will be None.
///
/// For the GroupInfo struct, the hash will be Some(blake3).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Key {
    /// Individual file size (in bytes)
    #[serde(serialize_with = "add_thousands_separator")]
    pub size: usize,
    /// Blake3 hash
    pub hash: Option<String>,
}

impl Key {
    pub fn new(value: u64, hash: Option<String>) -> Self {
        match value.try_into() {
            Ok(size) => Key { size, hash },
            Err(why) => {
                panic!("Error converting from u64 to usize: {why}")
            }
        }
    }

    /// Set hash value
    pub fn set_hash(&mut self, new_hash: Option<String>) {
        self.hash = new_hash;
    }
}

#[cfg(test)]
mod test_key {
    use super::*;

    /// cargo test -- --show-output test_new_key
    #[test]
    fn test_new_key() {
        let value: u64 = 12345;
        let valid = Key::new(value, None);
        let result = Key {
            size: 12345,
            hash: None,
        };
        println!("key: {valid:#?}");
        assert_eq!(valid, result);
    }

    /// cargo test -- --show-output test_set_hash
    #[test]
    fn test_set_hash() {
        let mut key = Key::new(123, None);
        println!("key: {key:#?}");

        let string = "foo bar".to_string();
        key.set_hash(Some(string.clone()));
        println!("key: {key:#?}");

        let result = Key {
            size: 123,
            hash: Some(string),
        };
        assert_eq!(key, result);
    }
}
