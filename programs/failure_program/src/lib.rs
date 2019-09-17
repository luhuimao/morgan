use morgan_interface::account::KeyedAccount;
use morgan_interface::instruction::InstructionError;
use morgan_interface::pubkey::Pubkey;
use morgan_interface::morgan_entrypoint;

morgan_entrypoint!(entrypoint);
fn entrypoint(
    _program_id: &Pubkey,
    _keyed_accounts: &mut [KeyedAccount],
    _data: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    Err(InstructionError::GenericError)
}
