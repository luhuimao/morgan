use log::*;
use crate::instruction::{AccountMeta, Instruction};
use crate::instruction_processor_utils::DecodeError;
use crate::pubkey::Pubkey;
use crate::system_program;
use num_derive::FromPrimitive;
use morgan_helper::logHelper::*;

#[derive(Serialize, Debug, Clone, PartialEq, FromPrimitive)]
pub enum SystemError {
    AccountAlreadyInUse,
    ResultWithNegativeDifs,
    SourceNotSystemAccount,
    ResultWithNegativeReputations,
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
        reputations: u64,
        space: u64,
        program_id: Pubkey,
    },
    /// Assign account to a program
    /// * Transaction::keys[0] - account to assign
    Assign { program_id: Pubkey },
    /// Transfer difs
    /// * Transaction::keys[0] - source
    /// * Transaction::keys[1] - destination
    Transfer { difs: u64 },
    /// Create a new account
    /// * Transaction::keys[0] - source
    /// * Transaction::keys[1] - new account key
    /// * reputations - number of reputations to transfer to the new account
    /// * space - memory to allocate if greater then zero
    /// * program_id - the program id of the new account
    CreateAccountWithReputation {
        reputations: u64,
        space: u64,
        program_id: Pubkey,
    },
    /// Transfer reputations
    /// * Transaction::keys[0] - source
    /// * Transaction::keys[1] - destination
    TransferReputations { reputations: u64 },
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
    let reputations = 0;
    Instruction::new(
        system_program::id(),
        &SystemInstruction::CreateAccount {
            difs,
            reputations,
            space,
            program_id: *program_id,
        },
        account_metas,
    )
}

pub fn create_account_with_reputation(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    reputations: u64,
    space: u64,
    program_id: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];
    // info!("{}", Info(format!("create_account_with_reputation: {:?}", account_metas).to_string()));
    println!("{}",
        printLn(
            format!("create_account_with_reputation: {:?}", account_metas).to_string(),
            module_path!().to_string()
        )
    );
    Instruction::new(
        system_program::id(),
        &SystemInstruction::CreateAccountWithReputation {
            reputations,
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

/// Create and sign a transaction to create a system account with reputation
pub fn create_user_account_with_reputation(from_pubkey: &Pubkey, to_pubkey: &Pubkey, reputations: u64) -> Instruction {
    let program_id = system_program::id();
    // info!("{}", Info(format!("create_user_account_with_reputation to : {:?}", to_pubkey).to_string()));
    println!("{}",
        printLn(
            format!("create_user_account_with_reputation to : {:?}", to_pubkey).to_string(),
            module_path!().to_string()
        )
    );
    create_account_with_reputation(from_pubkey, to_pubkey, reputations, 0, &program_id)
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

pub fn transfer_reputations(from_pubkey: &Pubkey, to_pubkey: &Pubkey, reputations: u64) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*from_pubkey, true),
        AccountMeta::new(*to_pubkey, false),
    ];
    Instruction::new(
        system_program::id(),
        &SystemInstruction::TransferReputations { reputations },
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
