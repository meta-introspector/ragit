use ragit_fs::{read_bytes, write_bytes, WriteMode};
pub use ragit_types::uid::{Uid, UidError, UidWriteMode};

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
    use ragit_types::uid::UidType;
    use std::str::FromStr;

    #[test]
    fn uid_from_str_test() {
        let uid_str = "0000000000000000000000000000000000000000000000000000000000000001";
        let uid = Uid::from_str(uid_str).unwrap();
        assert_eq!(uid.to_string(), uid_str);
    }

    #[test]
    fn uid_to_str_test() {
        let uid_str = "0000000000000000000000000000000000000000000000000000000000000001";
        let uid = Uid::from_str(uid_str).unwrap();
        assert_eq!(uid.to_string(), uid_str);
    }

    #[test]
    fn uid_xor_test() {
        let uid1 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000002").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000004").unwrap();
        let uid3 = uid1 ^ uid2;
        assert_eq!(uid3.to_string(), "0000000000000000000000000000000200000000000000000000000000000006");
    }

    #[test]
    fn uid_add_test() {
        let uid1 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000002").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000004").unwrap();
        let uid3 = uid1 + uid2;
        assert_eq!(uid3.to_string(), "0000000000000000000000000000000400000000000000000000000000000006");
    }

    #[test]
    fn uid_add_assign_test() {
        let mut uid1 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000002").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000004").unwrap();
        uid1 += uid2;
        assert_eq!(uid1.to_string(), "0000000000000000000000000000000400000000000000000000000000000006");
    }

    #[test]
    fn uid_checked_sub_test() {
        let uid1 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000004").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000002").unwrap();
        let uid3 = uid1.checked_sub(uid2).unwrap();
        assert_eq!(uid3.to_string(), "0000000000000000000000000000000200000000000000000000000000000002");
    }

    #[test]
    fn uid_checked_sub_test_2() {
        let uid1 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000002").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000004").unwrap();
        let uid3 = uid1.checked_sub(uid2).unwrap();
        assert_eq!(uid3.to_string(), "00000000000000000000000000000001fffffffffffffffffffffffffffffffe");
    }

    #[test]
    fn uid_checked_sub_fail_test() {
        let uid1 = Uid::from_str("0000000000000000000000000000000100000000000000000000000000000002").unwrap();
        let uid2 = Uid::from_str("0000000000000000000000000000000300000000000000000000000000000004").unwrap();
        assert!(uid1.checked_sub(uid2).is_none());
    }

    // Commenting out uid_prefix_suffix_test as from_prefix_and_suffix is not available publicly.
    // #[test]
    // fn uid_prefix_suffix_test() {
    //     let uid = Uid {
    //         high: 0x1234567890abcdef1234567890abcdef,
    //         low: 0xfedcba0987654321fedcba0987654321,
    //     };
    //     let prefix = uid.get_prefix();
    //     let suffix = uid.get_suffix();
    //     assert_eq!(prefix, "12");
    //     assert_eq!(
    //         suffix,
    //         "34567890abcdef1234567890abcdeffedcba0987654321fedcba0987654321"
    //     );
    //     let uid2 = Uid::from_prefix_and_suffix(&prefix, &suffix).unwrap();
    //     assert_eq!(uid, uid2);
    // }

    #[test]
    fn uid_abbrev_test() {
        let uid = Uid::from_str("1234567890abcdef1234567890abcdefedcba0987654321fedcba0987654321").unwrap();
        assert_eq!(uid.abbrev(8), "12345678");
    }

    #[test]
    fn uid_type_test() {
        let uid = Uid::new_from_slice(b"some data"); // This sets CHUNK_TYPE internally
        assert_eq!(uid.get_uid_type().unwrap(), UidType::Chunk);
    }

    #[test]
    fn uid_data_size_test() {
        let test_data = b"some data with a specific length";
        let uid = Uid::new_from_slice(test_data);
        assert_eq!(uid.get_data_size(), test_data.len());
    }
}
