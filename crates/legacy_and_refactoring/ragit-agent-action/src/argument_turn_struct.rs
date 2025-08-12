use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ArgumentTurn {
    // An argument of an action, that AI generated
    assistant: String,

    // If the argument is valid, it's a result of the action.
    // Otherwise, it's a feedback: why the argument is invalid and how to fix it.
    user: String,
}
