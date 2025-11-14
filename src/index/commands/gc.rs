use crate::index::index_struct::Index;
use crate::error::Error;
use crate::prelude::*;
use ragit_fs::{file_name, parent, remove_file};
use std::collections::HashSet;

pub fn gc(
    index: &mut Index,
    images: bool,
) -> Result<(), Error> {
    if images {
        let mut all_images = HashSet::new();

        for chunk_file in &index.get_all_chunk_files()? {
            for image in crate::chunk::load_from_file(&chunk_file)?.images {
                all_images.insert(image);
            }
        }

        for image_file in &index.get_all_image_files()? {
            let uid = Uid::from_prefix_and_suffix(
                &file_name(&parent(&image_file)?)?,
                &file_name(&image_file)?,
            )?;

            if !all_images.contains(&uid) {
                remove_file(&image_file)?;
                remove_file(&format!("{}.json", image_file.display()))?;
            }
        }
    }

    Ok(())
}
