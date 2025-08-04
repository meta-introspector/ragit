use super::action_state_struct::ActionState;
use ragit_types::schema::Schema;
use ragit_types::schema::schema_builders::{integer_between, default_yesno};


impl ActionState {
    pub fn get_schema(&self) -> Option<Schema> {
        if self.index.is_none() {
            Some(integer_between(
                Some(1),
                Some(self.actions.len() as i128),
            ))
        } else if !self.complete {
            None
        } else if self.r#continue.is_none() {
            Some(schema_builders::default_yesno())
        } else {
            unreachable!()
        }
    }
}
