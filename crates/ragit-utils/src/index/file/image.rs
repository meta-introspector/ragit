use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Image;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ImageDescription;

pub struct ImageReader;