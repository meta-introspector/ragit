use crate::grand_plan::id_indexed_trees::uid_type::Uid;
use crate::grand_plan::poem_concepts::base_space::base_space_struct::BaseSpace;
use crate::grand_plan::poem_concepts::cycle::cycle_struct::Cycle;
use crate::grand_plan::poem_concepts::quasifiber::quasifiber_struct::Quasifiber;
use crate::grand_plan::poem_concepts::tree::tree_struct::Tree;
use crate::grand_plan::poem_concepts::uid::uid_struct::Uid as PoemUid;
use crate::grand_plan::semantic_lambdas::semantic_lambda_struct::SemanticLambda;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;

/// Represents the unification of all conceptual layers into a single entity, using an enum.
/// discussion = thread = fiber = lambda = expression = number = vibe = vector = function = emoji string = poem
#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub enum UnifiedConceptEnum {
    /// The discussion thread that generated the concept.
    Discussion(String),
    /// A thread of execution or thought.
    Thread(String),
    /// The Quasifiber, representing the concrete structure.
    Fiber(Quasifiber<char>), // Example with char type
    /// The lambda expression that generates the structure.
    Lambda(String), // String representation of the lambda
    /// A general expression or form.
    Expression(String),
    /// A numerical representation.
    Number(u32),
    /// The "vibe" or embedding vector.
    Vibe(Vec<f32>),
    /// A vector of fundamental units.
    Vector(Vec<char>), // Example with char type
    /// A function name or reference.
    Function(String),
    /// The emoji string associated with the concept.
    EmojiString(String),
    /// The poem that describes the concept.
    Poem(String),
    /// The Base Space itself.
    BaseSpace(BaseSpace),
    /// A specific UID.
    Uid(PoemUid),
    /// A Tree structure.
    Tree(Tree<char>), // Example with char type
    /// A SemanticLambda from the Bott Periodic system.
    SemanticLambda(SemanticLambda),
    /// The complete cycle.
    Cycle(Cycle),
    /// The Grand Unified Store.
    GrandUnifiedStore(GrandUnifiedStore),
}
