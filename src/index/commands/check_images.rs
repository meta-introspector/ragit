use crate::prelude::*;
use image::ImageFormat;

pub(super) fn check_images(
    index: &Index,
    images: &mut HashMap<Uid, bool>,
) -> Result<(), Error> {
    for image_file in &index.get_all_image_files()? {
        let image_uid = Index::get_uid_path(
            path_utils::pathbuf_to_str(&index.root_dir),
            IMAGE_DIR_NAME,
            Uid::from_prefix_and_suffix(
                &file_name(path_utils::pathbuf_to_str(&parent(image_file.as_path())?))?,
                &file_name(path_utils::pathbuf_to_str(image_file))?,
            )?,
            Some("png"),
        )?;
        let image_description_path = path_utils::str_to_pathbuf(&set_extension(path_utils::pathbuf_to_str(image_file), "json")?);

        match images.get_mut(&image_uid.to_str().ok_or_else(|| Error::BrokenIndex(error_reporting::invalid_image_uid(path_utils::path_to_display(&image_uid))))?.parse::<Uid>()?) {
            Some(exists) => { *exists = true; },
            None => {
                // NOTE: it's not an error. see the comments above.
            },
        }

        let image_bytes = read_bytes(path_utils::pathbuf_to_str(image_file))?;
        image::load_from_memory_with_format(  // Check F
            &image_bytes,
            ImageFormat::Png,
        )?;
        let image_description = read_string(path_utils::pathbuf_to_str(&image_description_path))?;

        // Check F
        if serde_json::from_str::<ImageDescription>(&image_description).is_err() {
            return Err(Error::BrokenIndex(error_reporting::image_description_missing(path_utils::path_to_display(image_file), path_utils::path_to_display(&image_description_path))));
        }
    }

    for (image_uid, exists) in images.iter() {
        if !*exists {  // Check E
            return Err(Error::BrokenIndex(error_reporting::image_not_found(image_uid.to_string())));
        }
    }
    Ok(())
}
