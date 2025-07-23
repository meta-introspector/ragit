use crate::error::Error;
use crate::model::ModelRaw;
use rand::seq::SliceRandom;

pub fn get_selected_models(models: &[ModelRaw]) -> Result<Vec<ModelRaw>, Error> {
    let presupposed: Vec<ModelRaw> = models
        .iter()
        .filter(|m| {
            m.initial_score
                .as_ref()
                .map(|s| s.contains("presupposed"))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    let unknown: Vec<ModelRaw> = models
        .iter()
        .filter(|m| {
            m.initial_score
                .as_ref()
                .map(|s| s == "unknown")
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    let mut selected_models: Vec<ModelRaw> = Vec::new();

    if !presupposed.is_empty() {
        selected_models.push(presupposed.choose(&mut rand::thread_rng()).unwrap().clone());
    }
    if !unknown.is_empty() && selected_models.len() < 2 {
        selected_models.push(unknown.choose(&mut rand::thread_rng()).unwrap().clone());
    }
    if selected_models.len() < 2 && !presupposed.is_empty() {
        selected_models.push(
            presupposed
                .iter()
                .filter(|m| m.name != selected_models[0].name)
                .cloned()
                .collect::<Vec<ModelRaw>>()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone(),
        );
    }
    if selected_models.len() < 2 {
        // Fallback to random selection if not enough specific models
        let mut all_models = models.to_vec();
        all_models.shuffle(&mut rand::thread_rng());
        selected_models = all_models.into_iter().take(2).collect();
    }

    if selected_models.len() < 2 {
        return Err(Error::InsufficientModels);
    }
    Ok(selected_models)
}
