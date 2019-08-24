use morgan_sdk::account::KeyedAccount;
use morgan_sdk::instruction::InstructionError;
use morgan_sdk::pubkey::Pubkey;
use morgan_sdk::morgan_entrypoint;

morgan_entrypoint!(entrypoint);
fn entrypoint(
    _program_id: &Pubkey,
    _keyed_accounts: &mut [KeyedAccount],
    _data: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    Err(InstructionError::GenericError)
}
