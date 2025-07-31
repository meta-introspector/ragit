use super::{AtomicToken, FileReaderImpl};
use crate::error::Error;
use crate::index::BuildConfig;
use crate::prelude::*;
use ragit_fs::{extension, read_bytes};
use ragit_pdl::{Image, ImageType};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Cursor;

#[cfg(feature = "svg")]
use resvg::{
    render,
    tiny_skia::{Pixmap, Transform},
    usvg::{self, Tree},
};

pub type Path = String;


fn normalize_image(bytes: Vec<u8>, image_type: ImageType) -> Result<Vec<u8>, Error> {
    let mut dynamic_image = match image_type {
        ImageType::Svg => {
            #[cfg(feature = "svg")] {
                let bytes = render_svg_to_png(&bytes)?;
                image::load_from_memory_with_format(
                    &bytes,
                    ImageType::Png.try_into()?,
                )?
            }

            #[cfg(not(feature = "svg"))] {
                return Err(Error::FeatureNotEnabled { feature: String::from("svg"), action: String::from("read an svg file") });
            }
        },
        _ => image::load_from_memory_with_format(
            &bytes,
            image_type.try_into()?,
        )?,
    };

    if dynamic_image.width() > 1024 || dynamic_image.height() > 1024 {
        dynamic_image = dynamic_image.resize(1024, 1024, image::imageops::FilterType::Triangle);
    }

    // no modification at all
    else if image_type == ImageType::Png {
        return Ok(bytes);
    }

    let result = vec![];
    let mut writer = Cursor::new(result);
    dynamic_image.write_to(&mut writer, image::ImageFormat::Png)?;
    let result = writer.into_inner();

    Ok(result)
}

pub struct ImageReader {
    path: Path,
    tokens: Vec<AtomicToken>,
    image_type: ImageType,
    strict_mode: bool,
    is_exhausted: bool,
}

impl FileReaderImpl for ImageReader {
    fn new(path: &str, _root_dir: &str, config: &BuildConfig) -> Result<Self, Error> {
        Ok(ImageReader {
            path: path.to_string(),
            tokens: vec![],
            image_type: ImageType::from_extension(&extension(path)?.unwrap_or(String::new()))?,
            strict_mode: config.strict_file_reader,
            is_exhausted: false,
        })
    }

    fn load_tokens(&mut self) -> Result<(), Error> {
        if self.is_exhausted {
            Ok(())
        }

        else {
            let bytes = read_bytes(&self.path)?;

            match normalize_image(bytes.clone(), self.image_type) {
                Ok(bytes) => {
                    let uid = Uid::new_image(&bytes);
                    self.tokens.push(AtomicToken::Image(Image {
                        bytes,
                        image_type: ImageType::Png,
                        uid,
                    }));
                    self.is_exhausted = true;
                    Ok(())
                },
                Err(e) => if self.strict_mode {
                    Err(e)
                } else {
                    if let ImageType::Svg = self.image_type {
                        let s = String::from_utf8_lossy(&bytes).to_string();
                        self.tokens.push(AtomicToken::String {
                            data: s.clone(),
                            char_len: s.chars().count(),
                        });
                        self.is_exhausted = true;
                        Ok(())
                    }

                    else {
                        Err(e)
                    }
                },
            }
        }
    }

    fn pop_all_tokens(&mut self) -> Result<Vec<AtomicToken>, Error> {
        let mut result = vec![];
        std::mem::swap(&mut self.tokens, &mut result);
        Ok(result)
    }

    fn has_more_to_read(&self) -> bool {
        !self.is_exhausted
    }

    fn key(&self) -> String {
        String::from("image_reader_v0")
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct ImageDescription {
    pub extracted_text: String,
    pub explanation: String,
}

#[cfg(feature = "svg")]
fn render_svg_to_png(svg: &[u8]) -> Result<Vec<u8>, Error> {
    let tree_options = usvg::Options {
        // It returns `None` if `width` or `height` is negative.
        // So we can safely unwrap the result.
        default_size: usvg::Size::from_wh(1024.0, 1024.0).unwrap(),
        ..usvg::Options::default()
    };
    let tree = Tree::from_data(svg, &tree_options)?;
    let svg_size = tree.size();

    // As far as I know, it returns None only if size is 0
    let mut pixmap = Pixmap::new(
        svg_size.width() as u32,
        svg_size.height() as u32,
    ).unwrap_or_else(
        || Pixmap::new(1024, 1024).unwrap()
    );
    render(
        &tree,
        Transform::identity(),
        &mut pixmap.as_mut(),
    );

    Ok(pixmap.encode_png()?)
}
