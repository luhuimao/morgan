use crate::instruction::{AccountMeta, Instruction};
use crate::instruction_processor_utils::DecodeError;
use crate::pubkey::Pubkey;
use crate::system_program;
use num_derive::FromPrimitive;

#[derive(Serialize, Debug, Clone, PartialEq, FromPrimitive)]
pub enum SystemError {
    AccountAlreadyInUse,
    ResultWithNegativeDifs,
    SourceNotSystemAccount,
}

impl<T> DecodeError<T> for SystemError {
    fn type_of(&self) -> &'static str {
        "SystemError"
    }
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error")
    }
}

impl std::error::Error for SystemError {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SystemInstruction {
    /// Create a new account
    /// * Transaction::keys[0] - source
    /// * Transaction::keys[1] - new account key
    /// * difs - number of difs to transfer to the new account
    /// * space - memory to allocate if greater then zero
    /// * program_id - the program id of the new account
    CreateAccount {
        difs: u64,
        space: u64,
        program_id: Pubkey,
    },
    /// Assign account to a program
    /// * Transaction::keys[0] - account to assign
    Assign {
        program_id: Pubkey
    },
    /// Transfer difs
    /// * Transaction::keys[0] - source
    /// * Transaction::keys[1] - destination
    Transfer {
        difs: u64
    },
}

pub fn create_account(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    difs: u64,
    space: u64,
    program_id: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];
    Instruction::new(
        system_program::id(),
        &SystemInstruction::CreateAccount {
            difs,
            space,
            program_id: *program_id,
        },
        account_metas,
    )
}

/// Create and sign a transaction to create a system account
pub fn create_user_account(from_pubkey: &Pubkey, to_pubkey: &Pubkey, difs: u64) -> Instruction {
    let program_id = system_program::id();
    create_account(from_pubkey, to_pubkey, difs, 0, &program_id)
}

pub fn assign(from_pubkey: &Pubkey, program_id: &Pubkey) -> Instruction {
    let account_metas = vec![AccountMeta::new(*from_pubkey, true)];
    Instruction::new(
        system_program::id(),
        &SystemInstruction::Assign {
            program_id: *program_id,
        },
        account_metas,
    )
}

pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, difs: u64) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];
    Instruction::new(
        system_program::id(),
        &SystemInstruction::Transfer { difs },
        account_metas,
    )
}

/// Create and sign new SystemInstruction::Transfer transaction to many destinations
pub fn transfer_many(from_pubkey: &Pubkey, to_difs: &[(Pubkey, u64)]) -> Vec<Instruction> {
    to_difs
        .iter()
        .map(|(to_pubkey, difs)| transfer(from_pubkey, to_pubkey, *difs))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_keys(instruction: &Instruction) -> Vec<Pubkey> {
        instruction.accounts.iter().map(|x| x.pubkey).collect()
    }

    #[test]
    fn test_move_many() {
        let alice_pubkey = Pubkey::new_rand();
        let bob_pubkey = Pubkey::new_rand();
        let carol_pubkey = Pubkey::new_rand();
        let to_difs = vec![(bob_pubkey, 1), (carol_pubkey, 2)];

        let instructions = transfer_many(&alice_pubkey, &to_difs);
        assert_eq!(instructions.len(), 2);
        assert_eq!(get_keys(&instructions[0]), vec![alice_pubkey, bob_pubkey]);
        assert_eq!(get_keys(&instructions[1]), vec![alice_pubkey, carol_pubkey]);
    }
}
