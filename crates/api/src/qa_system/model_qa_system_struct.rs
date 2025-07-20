use crate::model::ModelRaw;

pub struct ModelQASystem {
    pub models: Vec<ModelRaw>,
    pub throttling_safety_margin: f64,
}
