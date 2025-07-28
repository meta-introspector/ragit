use super::action_enum::Action;
use super::action_state_struct::ActionState;

impl ActionState {
    pub fn new(actions: Vec<Action>) -> Self {
        ActionState {
            actions,
            complete: false,
            ..ActionState::default()
        }
    }
}
