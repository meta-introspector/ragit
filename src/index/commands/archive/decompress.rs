use crate::error::Error;

extern crate zstd;

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let mut decoder = zstd::Decoder::new(bytes)?;
    let mut decompressed = Vec::new();
    std::io::copy(&mut decoder, &mut decompressed)?;
    Ok(decompressed)
}
