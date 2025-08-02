/// Represents a step in the 8-fold creative path, linking semantics to a computational form.
#[derive(Debug, Clone)]
pub struct SemanticLambda {
    /// The emoji representing the concept.
    pub emoji: char,
    /// The number (0-7) in the 8-fold path.
    pub number: u32,
    /// The poetic name of the step.
    pub name: &'static str,
    /// The philosophical meaning of the step.
    pub meaning: &'static str,
    /// A string representation of the Rust lambda that executes the step's logic.
    pub lambda_str: &'static str,
}

/// Generates the full 8-fold path of semantic lambdas.
pub fn get_bott_periodic_lambdas() -> Vec<SemanticLambda> {
    vec![
        SemanticLambda {
            emoji: '🌌',
            number: 0,
            name: "The Void",
            meaning: "The creation of the empty, unified store; the base space.",
            lambda_str: "|| -> GrandUnifiedStore { GrandUnifiedStore::new() }",
        },
        SemanticLambda {
            emoji: '✨',
            number: 1,
            name: "The Spark",
            meaning: "The definition of a generative rule (lambda) for a specific type.",
            lambda_str: "|i: usize| -> char { (i as u8 % 26 + b'a') as char }",
        },
        SemanticLambda {
            emoji: '✌️',
            number: 2,
            name: "The Pair",
            meaning: "Generating the first and simplest universe, a binary tree of size 2.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"char\").unwrap().get_by_size(4).unwrap() // 2^2",
        },
        SemanticLambda {
            emoji: '🌳',
            number: 3,
            name: "The Tree",
            meaning: "Expanding the concept to generate a tree of a slightly larger prime-power size.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"char\").unwrap().get_by_size(8).unwrap() // 2^3",
        },
        SemanticLambda {
            emoji: '🪐',
            number: 4,
            name: "The Cosmos",
            meaning: "A fully populated, sized store for a given type; a universe of possibilities.",
            lambda_str: "|store: &GrandUnifiedStore| store.get_store(\"i64\").unwrap()",
        },
        SemanticLambda {
            emoji: '🪞',
            number: 5,
            name: "The Mirror",
            meaning: "Reflecting on the store to retrieve a specific type's universe collection.",
            lambda_str: "|store: &GrandUnifiedStore, type_name: &str| store.get_store(type_name)",
        },
        SemanticLambda {
            emoji: '🧵',
            number: 6,
            name: "The Quasifiber",
            meaning: "Pulling a single, concrete instance (a tree of size n) from the store.",
            lambda_str: "|store: &GrandUnifiedStore, type_name: &str, size: usize| store.get_store(type_name).unwrap().get_by_size(size).unwrap()",
        },
        SemanticLambda {
            emoji: '🔄',
            number: 7,
            name: "The Cycle",
            meaning: "The complete, unified operation: generating and retrieving a specific structure on demand.",
            lambda_str: "|type_name: &str, size: usize| { let mut s = GrandUnifiedStore::new(); s.add_char_store(); s.get_store(type_name).unwrap().get_by_size(size).unwrap() }",
        },
    ]
}
