use ragit_utils::error::Error;
use ragit_api::Model;

impl super::Index {
    pub(crate) fn get_model_by_name(&self, name: &str) -> Result<Model, Error> {
        Ok(ragit_api::get_model_by_name(&self.models, name)?.clone())
    }

    /// Finds the lowest-cost model in the loaded models.
    pub fn find_lowest_cost_model(&self) -> Option<&Model> {
        if self.models.is_empty() {
            return None;
        }

        self.models.iter().min_by(|a, b| {
            let a_cost =
                a.dollars_per_1b_input_tokens as u128 + a.dollars_per_1b_output_tokens as u128;
            let b_cost =
                b.dollars_per_1b_input_tokens as u128 + b.dollars_per_1b_output_tokens as u128;
            a_cost.cmp(&b_cost)
        })
    }
}