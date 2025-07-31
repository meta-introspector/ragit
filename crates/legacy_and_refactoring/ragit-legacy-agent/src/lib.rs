// use std::collections::HashMap;

// use ragit_api::Request;
// use ragit_fs::{WriteMode, join, write_string};
// use ragit_pdl::{Pdl, Schema, into_context, parse_pdl};
// use serde::{Deserialize, Serialize};
// use serde_json::Value;

// use crate::Uid;
// use crate::prelude::*;
// use ragit_pdl::Prompt;
// use ragit_index_query::QueryResponse;

// use ragit_agent_action::{
//     Action, ActionResult, ActionState, ActionTrace, ArgumentTurn, SearchType,
// };
// use ragit_agent::file_tree::FileTree;
// use ragit_types::chunk::chunk_struct::Chunk;
// use ragit_index_types::Index;
// use ragit_utils::path_utils;

// // `derive(Serialize) for AgentState` has 2 purposes.
// //
// // 1. Dump log in `.ragit/logs`.
// // 2. Create `tera::Context` for `agent.pdl`. The context is fed to the AI.
// //
// // I'm NOT gonna `derive(Deserialize)`. There are too many edge cases in
// // deserializing the state.
// #[derive(Debug, Serialize)]
// struct AgentState {
//     question: String,
//     context: String,
//     needed_information: Option<String>,
//     action_states: Vec<ActionState>,

//     // Actions that this agent can run.
//     // It's necessary because agents have different sets of capabilities.
//     // For example, the summary agent cannot run `Action::GetSummary` because
//     // there's no summary yet!
//     #[serde(skip)]
//     actions: Vec<Action>,

//     // The final response must be in this schema.
//     #[serde(skip)]
//     response_schema: Option<Schema>,

//     // It's generated from `actions`.
//     // It's fed to the AI's context.
//     action_prompt: String,

//     // I want to use the term `has_action_to_run`, but serde_json doesn't allow that :(
//     is_actions_complete: bool,
//     new_information: Option<String>,
//     new_context: Option<String>,

//     // when it `has_enough_information` it writes the
//     // final result to `response` field and break
//     has_enough_information: bool,
//     response: Option<String>,
// }

// impl AgentState {
//     pub fn get_schema(&self) -> Option<Schema> {
//         if self.has_enough_information {
//             self.response_schema.clone()
//         } else if self.needed_information.is_none() {
//             None
//         } else if self.has_action_to_run() {
//             self.action_states.last().unwrap().get_schema()
//         } else {
//             None
//         }
//     }

//     pub async fn update(
//         &mut self,
//         input: Value,
//         index: &Index,
//         action_traces: &mut Vec<ActionTrace>,
//     ) -> Result<(), Error> {
//         if self.has_enough_information {
//             self.response = Some(input.as_str().unwrap().to_string());
//         } else if self.needed_information.is_none() {
//             self.needed_information = Some(input.as_str().unwrap().to_string());
//             self.action_states
//                 .push(ActionState::new(self.actions.clone()));
//         } else if self.has_action_to_run() {
//             let last_action = self.action_states.last_mut().unwrap();
//             last_action.update(input, index, action_traces).await?;

//             match last_action.r#continue.as_ref().map(|s| s.as_str()) {
//                 Some("yes") => {
//                     self.action_states
//                         .push(ActionState::new(self.actions.clone()));
//                 }
//                 Some("no") => {
//                     self.is_actions_complete = true;
//                 }
//                 Some(s) => panic!("something went wrong: {s:?}"),
//                 _ => {}
//             }
//         } else if self.new_information.is_none() {
//             self.new_information = Some(input.as_str().unwrap().to_string());
//         } else if self.new_context.is_none() {
//             self.new_context = Some(input.as_str().unwrap().to_string());
//         } else {
//             unreachable!()
//         }

//         Ok(())
//     }

//     pub fn update_context(&mut self, context: String) {
//         self.context = context;
//         self.needed_information = None;
//         self.action_states = vec![];
//         self.is_actions_complete = false;
//         self.new_information = None;
//         self.new_context = None;
//         self.has_enough_information = false;
//         self.response = None;
//     }

//     fn has_action_to_run(&self) -> bool {
//         if let Some(action) = self.action_states.last() {
//             action.r#continue.as_ref().map(|s| s.as_str()) != Some("no")
//         } else {
//             true
//         }
//     }
// }

// impl Default for AgentState {
//     fn default() -> Self {
//         AgentState {
//             question: String::new(),
//             context: String::new(),
//             needed_information: None,
//             actions: vec![],
//             response_schema: None,
//             action_prompt: String::new(),
//             action_states: vec![],
//             is_actions_complete: false,
//             new_information: None,
//             new_context: None,
//             has_enough_information: false,
//             response: None,
//         }
//     }
// }

// #[derive(Serialize)]
// pub struct AgentResponse {
//     pub response: String,
//     pub actions: Vec<ActionTrace>,
// }

// impl AgentResponse {
//     pub fn retrieved_chunks(&self, index: &Index) -> Result<Vec<Chunk>, Error> {
//         let mut chunks = HashMap::new();

//         for action in self.actions.iter() {
//             match &action.result {
//                 ActionResult::ReadFileShort { chunk_uids, .. } => {
//                     for chunk_uid in chunk_uids.iter() {
//                         chunks.insert(*chunk_uid, index.get_chunk_by_uid(*chunk_uid)?);
//                     }
//                 }
//                 ActionResult::SimpleRag(rag) => {
//                     for chunk in rag.retrieved_chunks.iter() {
//                         chunks.insert(chunk.uid, chunk.clone());
//                     }
//                 }
//                 ActionResult::ReadChunk(chunk) => {
//                     chunks.insert(chunk.uid, chunk.clone());
//                 }

//                 // `ReadFileLong` and `Search` have chunks, but many of them are
//                 // irrelevant. The AI will likely to choose relevant ones from them
//                 // and call `Action::ReadChunk`, which will be counted.
//                 ActionResult::ReadFileLong(_)
//                 | ActionResult::NoSuchFile { .. }
//                 | ActionResult::ReadDir(_)
//                 | ActionResult::NoSuchDir { .. }
//                 | ActionResult::NoSuchChunk(_)
//                 | ActionResult::ReadChunkAmbiguous { .. }
//                 | ActionResult::ReadChunkTooMany { .. }
//                 | ActionResult::Search { .. }
//                 | ActionResult::GetMeta { .. }
//                 | ActionResult::NoSuchMeta { .. }
//                 | ActionResult::GetSummary(_) => {}
//             }
//         }

//         Ok(chunks.into_values().collect())
//     }
// }

pub fn dummy_legacy_agent() -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!("FIX ME LATER: Fix the bootstrap first and this code later."))
}