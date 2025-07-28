use super::action_state_struct::ActionState;
use ragit_api::Schema;

impl ActionState {
    pub fn get_schema(&self) -> Option<Schema> {
        if self.index.is_none() {
            Some(Schema::integer_between(
                Some(1),
                Some(self.actions.len() as i128),
            ))
        } else if !self.complete {
            None
        } else if self.r#continue.is_none() {
            Some(Schema::default_yesno())
        } else {
            unreachable!()
        }
    }
}
