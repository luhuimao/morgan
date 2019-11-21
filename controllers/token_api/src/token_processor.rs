use crate::token_state::TokenState;
use log::*;
use morgan_interface::account::KeyedAccount;
use morgan_interface::instruction::InstructionError;
use morgan_interface::pubkey::Pubkey;
use morgan_helper::logHelper::*;

pub fn process_instruction(
    program_id: &Pubkey,
    info: &mut [KeyedAccount],
    input: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    morgan_logger::setup();

    TokenState::process(program_id, info, input).map_err(|e| {
        // error!("{}", Error(format!("error: {:?}", e).to_string()));
        println!(
            "{}",
            Error(
                format!("error: {:?}", e).to_string(),
                module_path!().to_string()
            )
        );
        InstructionError::CustomError(e as u32)
    })
}
