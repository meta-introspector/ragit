use ragit_fs::{read_bytes, write_bytes, WriteMode};
pub use ragit_types::uid::{Uid, UidType, UidWriteMode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UidError {
    #[error("Invalid UID: {0}")]
    InvalidUid(String),
    #[error("Cannot decode u128 from bytes")]
    DecodeError,
    #[error(transparent)]
    Io(#[from] ragit_fs::FileError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub fn load_from_file(path: &std::path::PathBuf) -> Result<Vec<Uid>, UidError> {
    let bytes = read_bytes(path.to_str().unwrap())?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn save_to_file(
    path: &std::path::PathBuf,
    uids: &Vec<Uid>,
    write_mode: UidWriteMode,
) -> Result<(), UidError> {
    let bytes = serde_json::to_vec_pretty(uids)?;
    match write_mode {
        UidWriteMode::Naive => {
            write_bytes(path.to_str().unwrap(), &bytes, WriteMode::AlwaysCreate)?
        }
        UidWriteMode::Compact => {
            write_bytes(path.to_str().unwrap(), &bytes, WriteMode::AlwaysCreate)?
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Sha3_256};
    use std::fmt;
    use std::str::FromStr;

    // The Uid struct and its impls are now in ragit-types, so we need to re-implement
    // the test-specific parts or adjust the tests to use the Uid from ragit-types.
    // For simplicity, I'll re-implement the necessary parts for testing here.

    fn u128_from_bytes(bytes: &[u8]) -> Result<u128, UidError> {
        match bytes.len() {
            0 => Ok(0),
            1..=15 => {
                let mut padded = [0; 16];
                padded[(16 - bytes.len())..].copy_from_slice(bytes);
                Ok(u128::from_be_bytes(padded))
            }
            16 => Ok(u128::from_be_bytes(bytes.try_into().unwrap())),
            _ => Err(UidError::DecodeError),
        }
    }

    #[test]
    fn uid_from_str_test() {
        let uid = Uid::from_str("0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
        assert_eq!(uid.high, 0);
        assert_eq!(uid.low, 1);
    }

    #[test]
    fn uid_to_str_test() {
        let uid = Uid { high: 0, low: 1 };
        assert_eq!(
            uid.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );
    }

    #[test]
    fn uid_xor_test() {
        let uid1 = Uid { high: 1, low: 2 };
        let uid2 = Uid { high: 3, low: 4 };
        let uid3 = uid1 ^ uid2;
        assert_eq!(uid3.high, 2);
        assert_eq!(uid3.low, 6);
    }

    #[test]
    fn uid_add_test() {
        let uid1 = Uid { high: 1, low: 2 };
        let uid2 = Uid { high: 3, low: 4 };
        let uid3 = uid1 + uid2;
        assert_eq!(uid3.high, 4);
        assert_eq!(uid3.low, 6);
    }

    #[test]
    fn uid_add_assign_test() {
        let mut uid1 = Uid { high: 1, low: 2 };
        let uid2 = Uid { high: 3, low: 4 };
        uid1 += uid2;
        assert_eq!(uid1.high, 4);
        assert_eq!(uid1.low, 6);
    }

    #[test]
    fn uid_checked_sub_test() {
        let uid1 = Uid { high: 3, low: 4 };
        let uid2 = Uid { high: 1, low: 2 };
        let uid3 = uid1.checked_sub(uid2).unwrap();
        assert_eq!(uid3.high, 2);
        assert_eq!(uid3.low, 2);
    }

    #[test]
    fn uid_checked_sub_test_2() {
        let uid1 = Uid { high: 3, low: 2 };
        let uid2 = Uid { high: 1, low: 4 };
        let uid3 = uid1.checked_sub(uid2).unwrap();
        assert_eq!(uid3.high, 1);
        assert_eq!(uid3.low, u128::MAX - 1);
    }

    #[test]
    fn uid_checked_sub_fail_test() {
        let uid1 = Uid { high: 1, low: 2 };
        let uid2 = Uid { high: 3, low: 4 };
        assert!(uid1.checked_sub(uid2).is_none());
    }

    #[test]
    fn uid_prefix_suffix_test() {
        let uid = Uid {
            high: 0x1234567890abcdef1234567890abcdef,
            low: 0xfedcba0987654321fedcba0987654321,
        };
        let prefix = uid.get_prefix();
        let suffix = uid.get_suffix();
        assert_eq!(prefix, "12");
        assert_eq!(
            suffix,
            "34567890abcdef1234567890abcdeffedcba0987654321fedcba0987654321"
        );
        let uid2 = Uid::from_prefix_and_suffix(&prefix, &suffix).unwrap();
        assert_eq!(uid, uid2);
    }

    #[test]
    fn uid_abbrev_test() {
        let uid = Uid {
            high: 0x1234567890abcdef1234567890abcdef,
            low: 0xfedcba0987654321fedcba0987654321,
        };
        assert_eq!(uid.abbrev(8), "12345678");
    }

    #[test]
    fn uid_type_test() {
        let mut uid = Uid { high: 0, low: 0 };
        uid.low |= Uid::CHUNK_TYPE;
        assert_eq!(uid.get_uid_type().unwrap(), UidType::Chunk);
    }

    #[test]
    fn uid_data_size_test() {
        let mut uid = Uid { high: 0, low: 0 };
        uid.low |= 1234;
        assert_eq!(uid.get_data_size(), 1234);
    }
}
