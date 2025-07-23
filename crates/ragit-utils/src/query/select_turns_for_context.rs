use super::{MultiTurnSchema, QueryTurn};

pub fn select_turns_for_context(history: &[QueryTurn], query: &str) -> Vec<String> {
    match history.len() {
        0 => unreachable!(),
        1 => vec![
            history[0].query.to_string(),
            history[0].response.response.to_string(),
            query.to_string(),
        ],
        _ => {
            let last_turn = history.last().unwrap();

            match &last_turn.response.multi_turn_schema {
                None => vec![
                    last_turn.query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
                // use rephrased query if in-context
                Some(MultiTurnSchema {
                    is_query: true,
                    in_context: true,
                    query: rephrased_query,
                }) => vec![
                    rephrased_query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
                // still in context, but is not a query (e.g. greetings)
                Some(MultiTurnSchema {
                    is_query: false,
                    in_context: true,
                    query: _,
                }) => {
                    let before_last_turn = history.get(history.len() - 2).unwrap();

                    vec![
                        before_last_turn.query.to_string(),
                        before_last_turn.response.response.to_string(),
                        last_turn.query.to_string(),
                        last_turn.response.response.to_string(),
                        query.to_string(),
                    ]
                }
                // start a new context
                Some(MultiTurnSchema {
                    in_context: false, ..
                }) => vec![
                    last_turn.query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
            }
        }
    }
}
