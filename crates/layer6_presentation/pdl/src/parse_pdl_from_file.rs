use crate::error::Error;
use crate::Pdl;
use ragit_fs::{read_string, parent};
use std::path::Path;

pub fn parse_pdl_from_file(
    path: &str,
    context: &tera::Context,
    strict_mode: bool,
) -> Result<Pdl, Error> {
    super::parse_pdl::parse_pdl(
        &read_string(path)?,
        context,
        parent(Path::new(path))?.to_str().unwrap(),
        strict_mode,
    )
}
