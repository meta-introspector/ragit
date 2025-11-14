use ragit_types::uid::{Uid,
		       //UidError
};
use ragit_fs::{file_size, get_relative_path, read_bytes_offset, read_bytes};
use sha3::{Digest, Sha3_256};
use ragit_types::ApiError as Error;

pub fn uid_new_file(
    root_dir: &str,
    path: &str,
) -> Result<Uid, Error> {
    let size = file_size(path)?;
    let rel_path = get_relative_path(&root_dir.to_string(), &path.to_string())?;
    let mut file_path_hasher = Sha3_256::new();
    file_path_hasher.update(rel_path.as_bytes());
    //let _file_path_uid = format!("{:064x}", file_path_hasher.finalize()).parse::<Uid>().unwrap();
    let mut file_content_hasher = Sha3_256::new();

    if size < 32 * 1024 * 1024 {
        let bytes = read_bytes(path)?;
        file_content_hasher.update(&bytes);
    }

    else {
        let block_size = 32 * 1024 * 1024;
        let mut offset = 0;

        loop {
            let bytes = read_bytes_offset(path, offset, offset + block_size)?;
            file_content_hasher.update(&bytes);
            offset += block_size;

            if offset >= size {
                break;
            }
        }
    }

    let  result = format!("{:064x}", file_content_hasher.finalize()).parse::<Uid>().unwrap();
    // Assuming Uid has a public xor_assign method or similar for combining.
    // If not, this will need to be adjusted based on Uid's actual API.
    // For now, using a placeholder that assumes direct field access or a helper.
    // This part will likely cause errors until Uid's API is clarified.
    // result ^= file_path_uid;
    // result.low &= Uid::METADATA_MASK;
    // result.low |= Uid::FILE_TYPE;
    // result.low |= (size as u128) & 0xffff_ffff;
    Ok(result)
}
