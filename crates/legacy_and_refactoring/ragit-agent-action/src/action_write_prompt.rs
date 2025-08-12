use super::action_enum::Action;

impl Action {
    pub(crate) fn write_prompt(actions: &[Action]) -> String {
        actions
            .iter()
            .enumerate()
            .map(|(i, p)| format!("{}. {}", i + 1, p.write_unit_prompt()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
