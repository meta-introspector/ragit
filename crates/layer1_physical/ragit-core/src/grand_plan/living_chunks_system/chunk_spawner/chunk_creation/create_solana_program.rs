use crate::grand_plan::poem_concepts::quasifiber::quasifiber_struct::Quasifiber;
use crate::grand_plan::solana_integration::solana_program_concept::solana_program_concept_struct::SolanaProgram;

pub fn create_solana_program() -> SolanaProgram {
    // Create a dummy Quasifiber for the new SolanaProgram
    let dummy_quasifiber: Quasifiber<char> = Quasifiber(crate::grand_plan::binary_id_trees::universe_struct::Universe::new());
    let solana_program: SolanaProgram = dummy_quasifiber.into();
    solana_program
}
