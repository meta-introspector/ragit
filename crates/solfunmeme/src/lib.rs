use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("\n\x1b[91m  /\\_\\  AI Fungus Growth Sim\n \\_\\/  \\ \x1b[33m*\x1b[0m\n \x1b[91m ( \\ ) \x1b[33m*\x1b[0m\n\x1b[91m  / \\ \n /___\\ \x1b[33m*\x1b[0m\n\x1b[0m");
    Ok(())
}
