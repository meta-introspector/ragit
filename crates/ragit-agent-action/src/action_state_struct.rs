use serde::Serialize;
use super::action_enum::Action;
use super::action_result_enum::ActionResult;
use super::argument_turn_struct::ArgumentTurn;

// The primary goal of this struct is to render `agent.pdl`.
#[derive(Debug, Default, Serialize)]
pub struct ActionState {
    // A set of actions that it can run.
    #[serde(skip)]
    pub actions: Vec<Action>,

    // It uses an index of an action instead of the action itself.
    // That's because it's tricky to (de)serialize actions.
    pub index: Option<usize>,
    pub instruction: Option<String>,

    // It might take multiple turns for the AI to generate an argument.
    // e.g. if it tries to read a file that does not exist, the engine
    // will give a feedback and the AI will retry
    pub argument_turns: Vec<ArgumentTurn>,

    // There's a valid argument in `argument_turns`, and it's run.
    pub complete: bool,

    pub result: Option<ActionResult>,

    // If yes, it runs another action within the same context
    pub r#continue: Option<String>, // "yes" | "no"
}
