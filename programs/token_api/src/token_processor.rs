use crate::token_state::TokenState;
use log::*;
use morgan_sdk::account::KeyedAccount;
use morgan_sdk::instruction::InstructionError;
use morgan_sdk::pubkey::Pubkey;

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
