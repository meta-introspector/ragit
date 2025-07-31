
use ragit_types::pdl_error::PdlError as Error;
use crate::pdl_struct::Pdl;
use ragit_fs::{read_string, parent};
use std::path::Path;
use crate::parse_pdl::parse_pdl_logic::parse_pdl_logic;

pub fn parse_pdl_from_file(
    path: &str,
    context: &tera::Context,
    strict_mode: bool,
) -> Result<Pdl, Error> {
    parse_pdl_logic(
        &read_string(path)?,
        context,
        parent(Path::new(path))?.to_str().unwrap(),
        strict_mode,
    )
}
