use crate::index::commands::version::VersionInfo;
use crate::index::index_struct::Index;
use crate::error::Error;
use crate::Path;

pub fn check_ragit_version(root_dir: &Path) -> Result<VersionInfo, Error> {
    let index = Index::load_minimum(root_dir.into())?;
    Ok(index.get_ragit_version_info())
}

pub fn migrate(
    index: &mut Index,
    to_version: String,
) -> Result<(), Error> {
    index.migrate(to_version)
}
