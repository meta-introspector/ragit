use crate::grand_plan::semantic_lambdas::semantic_lambda_struct::SemanticLambda;

/// Generates the full 8-fold path of semantic lambdas.
pub fn get_bott_periodic_lambdas() -> Vec<SemanticLambda> {
    vec![
        SemanticLambda {
            emoji: '🌌',
            number: 0,
            name: "the_void",
            meaning: "The creation of the empty, unified store; the base space.",
            lambda_str: "|| -> GrandUnifiedStore { GrandUnifiedStore::new() }",
        },
        SemanticLambda {
            emoji: '✨',
            number: 1,
            name: "the_spark",
            meaning: "The definition of a generative rule (lambda) for a specific type.",
            lambda_str: "|i: usize| -> char { (i as u8 % 26 + b'a') as char }",
        },
        SemanticLambda {
            emoji: "✌️",
            number: 2,
            name: "the_pair",
            meaning: "Generating the first and simplest universe, a binary tree of size 2.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"char\").unwrap().get_by_size(4).unwrap() // 2^2",
        },
        SemanticLambda {
            emoji: '🌳',
            number: 3,
            name: "the_tree",
            meaning: "Expanding the concept to generate a tree of a slightly larger prime-power size.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"char\").unwrap().get_by_size(8).unwrap() // 2^3",
        },
        SemanticLambda {
            emoji: '🪐',
            number: 4,
            name: "the_cosmos",
            meaning: "A fully populated, sized store for a given type; a universe of possibilities.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"i64\").unwrap()",
        },
        SemanticLambda {
            emoji: '🪞',
            number: 5,
            name: "the_mirror",
            meaning: "Reflecting on the store to retrieve a specific type's universe collection.",
            lambda_str: "|store: &GrandUnifiedStore, type_name: &str| store.get_store(type_name)",
        },
        SemanticLambda {
            emoji: '🧵',
            number: 6,
            name: "the_quasifiber",
            meaning: "Pulling a single, concrete instance (a tree of size n) from the store.",
            lambda_str: "|store: &GrandUnifiedStore, type_name: &str, size: usize| store.get_store(type_name).unwrap().get_by_size(size).unwrap()",
        },
        SemanticLambda {
            emoji: '🔄',
            number: 7,
            name: "the_cycle",
            meaning: "The complete, unified operation: generating and retrieving a specific structure on demand.",
            lambda_str: "|type_name: &str, size: usize| { let mut s = GrandUnifiedStore::new(); s.add_char_store(); s.get_store(type_name).unwrap().get_by_size(size).unwrap() }",
        },
    ]
}