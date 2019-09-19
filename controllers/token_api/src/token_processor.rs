use crate::token_state::TokenState;
use log::*;
use morgan_interface::account::KeyedAccount;
use morgan_interface::instruction::InstructionError;
use morgan_interface::pubkey::Pubkey;

pub fn process_instruction(
    program_id: &Pubkey,
    info: &mut [KeyedAccount],
    input: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    morgan_logger::setup();

    TokenState::process(program_id, info, input).map_err(|e| {
        error!("error: {:?}", e);
        InstructionError::CustomError(e as u32)
    })
}
