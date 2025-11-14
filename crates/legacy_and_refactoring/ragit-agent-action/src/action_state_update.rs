use super::action_state_struct::ActionState;
use super::action_trace_struct::ActionTrace;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use serde_json::Value;
use anyhow::Result;

impl ActionState {
    pub async fn update(
        &mut self,
        _input: Value,
        _index: &Index,
        _action_traces: &mut Vec<ActionTrace>,
    ) -> Result<(), ApiError> {
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
        // if self.index.is_none() {
        //     // If `input.as_u64()` fails, that means the AI is so stupid
        //     // that it cannot choose a number even with pdl schema's help.
        //     // So we just choose an arbitrary action. The AI's gonna fail
        //     // anyway and will break soon.
        //     let n = input.as_u64().unwrap_or(1) as usize;
        //     let action = self.actions[n - 1]; // AI uses 1-based index
        //     self.index = Some(n);
        //     self.instruction = Some(action.get_instruction(index)?);

        //     if !action.requires_argument() {
        //         // See comments in `Action::get_instruction`
        //         let argument = constants::ARGUMENT_OKAY;
        //         let result = action.run("", index).await?;
        //         let mut result_rendered = result.render();

        //         if !result.has_to_retry() {
        //             self.complete = true;
        //         }
        //         // If it's not complete, we have to give the instruction again so that the AI
        //         // will generate the argument.
        //         else {
        //             result_rendered = format!("{} {} {}", constants::FORMAT_RESULT_RENDERED, result_rendered, action.get_instruction(index)?);
        //         }

        //         self.argument_turns.push(ArgumentTurn {
        //             assistant: argument.to_string(),
        //             user: result_rendered.to_string(),
        //         });
        //         self.result = Some(result.clone());
        //         action_traces.push(ActionTrace {
        //             action,
        //             argument: None,
        //             result: result.clone(),
        //         });
        //     }
        // } else if !self.complete {
        //     let action = self.actions[self.index.unwrap() - 1]; // AI uses 1-based index

        //     // NOTE: pdl schema `string` is infallible
        //     let argument = input.as_str().unwrap();
        //     let result = action.run(argument, index).await?;
        //     let mut result_rendered = result.render();

        //     // Some AIs are not smart enough to generate a valid argument.
        //     // If the AI fails to generate valid argument more than once,
        //     // it just breaks.
        //     if !result.has_to_retry() || self.argument_turns.len() > 0 {
        //         self.complete = true;
        //     } else {
        //         result_rendered = format!("{} {} {}", constants::FORMAT_RESULT_RENDERED, result_rendered, action.get_instruction(index)?);
        //     }

        //     self.argument_turns.push(ArgumentTurn {
        //         assistant: argument.to_string(),
        //         user: result_rendered.to_string(),
        //     });
        //     self.result = Some(result.clone());
        //     action_traces.push(ActionTrace {
        //         action,
        //         argument: None,
        //         result: result.clone(),
        //     });
        // } else if self.r#continue.is_none() {
        //     // If `input.as_bool()` fails, that means the AI is
        //     // not smart enough to generate a boolean. There's
        //     // no need to continue.
        //     let input = input.as_bool().unwrap_or(false);
        //     let s = if input { constants::YES } else { constants::NO };

        //     self.r#continue = Some(s.to_string());
        // } else {
        //     unreachable!()
        // }

        // Ok(())
    }
}