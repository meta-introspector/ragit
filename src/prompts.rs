use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROMPTS: HashMap<String, String> = {
        let mut result = HashMap::new();
        result.insert(
            String::from("answer_query"),
            include_str!("../prompts/answer_query.pdl").to_string(),
        );
        result.insert(
            String::from("describe_image"),
            include_str!("../prompts/describe_image.pdl").to_string(),
        );
        result.insert(
            String::from("extract_keyword"),
            include_str!("../prompts/extract_keyword.pdl").to_string(),
        );
        result.insert(
            String::from("multi_turn"),
            include_str!("../prompts/multi_turn.pdl").to_string(),
        );
        result.insert(
            String::from("raw"),
            include_str!("../prompts/raw.pdl").to_string(),
        );
        result.insert(
            String::from("rerank_summary"),
            include_str!("../prompts/rerank_summary.pdl").to_string(),
        );
        result.insert(
            String::from("summarize"),
            include_str!("../prompts/summarize.pdl").to_string(),
        );

        // This prompt is deprecated and will be removed in 0.5.0.
        // result.insert(
        //     String::from("summarize_chunks"),
        //     include_str!("../prompts/summarize_chunks.pdl").to_string(),
        // );

        result.insert(
            String::from("agent"),
            include_str!("../prompts/agent.pdl").to_string(),
        );

        result.insert(
            String::from("muse_calliope"),
            include_str!("../prompts/muse_calliope.pdl").to_string(),
        );
        result.insert(
            String::from("muse_clio"),
            include_str!("../prompts/muse_clio.pdl").to_string(),
        );
        result.insert(
            String::from("muse_erato"),
            include_str!("../prompts/muse_erato.pdl").to_string(),
        );
        result.insert(
            String::from("muse_euterpe"),
            include_str!("../prompts/muse_euterpe.pdl").to_string(),
        );
        result.insert(
            String::from("muse_melpomene"),
            include_str!("../prompts/muse_melpomene.pdl").to_string(),
        );
        result.insert(
            String::from("muse_polyhymnia"),
            include_str!("../prompts/muse_polyhymnia.pdl").to_string(),
        );
        result.insert(
            String::from("muse_terpsichore"),
            include_str!("../prompts/muse_terpsichore.pdl").to_string(),
        );
        result.insert(
            String::from("muse_thalia"),
            include_str!("../prompts/muse_thalia.pdl").to_string(),
        );
        result.insert(
            String::from("muse_urania"),
            include_str!("../prompts/muse_urania.pdl").to_string(),
        );

        result
    };
}
