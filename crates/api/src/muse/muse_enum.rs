use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq, Hash, Display)]
pub enum MuseName {
    Calliope,
    Clio,
    Erato,
    Euterpe,
    Melpomene,
    Polyhymnia,
    Terpsichore,
    Thalia,
    Urania,
}
