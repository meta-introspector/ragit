use crate::error::Error;
use crate::index::index_struct::Index;

pub fn ii_build(
    index: &mut Index,
    quiet: bool,
) -> Result<(), Error> {
    index.build_ii(quiet)
}
