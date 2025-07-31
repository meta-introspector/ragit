use crate::error::Error;
use crate::index::index_struct::Index;

pub fn models_init(
    index: &mut Index,
) -> Result<(), Error> {
    index.load_or_init_models()
}
