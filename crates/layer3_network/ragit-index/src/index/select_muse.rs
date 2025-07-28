use ragit_api::MuseName;
use strum::IntoEnumIterator;

pub fn select_muse(index: usize) -> MuseName {
    let muses: Vec<MuseName> = MuseName::iter().collect();
    muses[index % muses.len()].clone()
}
