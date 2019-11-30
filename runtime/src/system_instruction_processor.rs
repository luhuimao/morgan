use log::*;
use morgan_interface::account::KeyedAccount;
use morgan_interface::instruction::InstructionError;
use morgan_interface::pubkey::Pubkey;
use morgan_interface::system_instruction::{SystemError, SystemInstruction};
use morgan_interface::system_program;

const FROM_ACCOUNT_INDEX: usize = 0;
const TO_ACCOUNT_INDEX: usize = 1;

fn create_system_account(
    keyed_accounts: &mut [KeyedAccount],
    difs: u64,
    reputations: u64,
    space: u64,
    program_id: &Pubkey,
) -> Result<(), SystemError> {
    if !system_program::check_id(&keyed_accounts[FROM_ACCOUNT_INDEX].account.owner) {
        debug!("CreateAccount: invalid account[from] owner");
        Err(SystemError::SourceNotSystemAccount)?;
    }

    if !keyed_accounts[TO_ACCOUNT_INDEX].account.data.is_empty()
        || !system_program::check_id(&keyed_accounts[TO_ACCOUNT_INDEX].account.owner)
    {
        debug!(
            "CreateAccount: invalid argument; account {} already in use",
            keyed_accounts[TO_ACCOUNT_INDEX].unsigned_key()
        );
        Err(SystemError::AccountAlreadyInUse)?;
    }
    if difs > keyed_accounts[FROM_ACCOUNT_INDEX].account.difs {
        debug!(
            "CreateAccount: insufficient difs ({}, need {})",
            keyed_accounts[FROM_ACCOUNT_INDEX].account.difs, difs
        );
        Err(SystemError::ResultWithNegativeDifs)?;
    }
    keyed_accounts[FROM_ACCOUNT_INDEX].account.difs -= difs;
    keyed_accounts[TO_ACCOUNT_INDEX].account.difs += difs;
    keyed_accounts[TO_ACCOUNT_INDEX].account.reputations += reputations;
    keyed_accounts[TO_ACCOUNT_INDEX].account.owner = *program_id;
    keyed_accounts[TO_ACCOUNT_INDEX].account.data = vec![0; space as usize];
    keyed_accounts[TO_ACCOUNT_INDEX].account.executable = false;
    Ok(())
}

fn create_system_account_with_reputation(
    keyed_accounts: &mut [KeyedAccount],
    reputations: u64,
    space: u64,
    program_id: &Pubkey,
) -> Result<(), SystemError> {
    if !system_program::check_id(&keyed_accounts[FROM_ACCOUNT_INDEX].account.owner) {
        debug!("CreateAccount: invalid account[from] owner");
        Err(SystemError::SourceNotSystemAccount)?;
    }

    if !keyed_accounts[TO_ACCOUNT_INDEX].account.data.is_empty()
        || !system_program::check_id(&keyed_accounts[TO_ACCOUNT_INDEX].account.owner)
    {
        debug!(
            "CreateAccount: invalid argument; account {} already in use",
            keyed_accounts[TO_ACCOUNT_INDEX].unsigned_key()
        );
        Err(SystemError::AccountAlreadyInUse)?;
    }
    if 1 > keyed_accounts[FROM_ACCOUNT_INDEX].account.difs {
        debug!(
            "CreateAccount: insufficient difs ({}, need {})",
            keyed_accounts[FROM_ACCOUNT_INDEX].account.difs, 1
        );
        Err(SystemError::ResultWithNegativeDifs)?;
    }
    keyed_accounts[FROM_ACCOUNT_INDEX].account.difs -= 1;
    keyed_accounts[TO_ACCOUNT_INDEX].account.difs += 1;
    keyed_accounts[TO_ACCOUNT_INDEX].account.reputations += reputations;
    keyed_accounts[TO_ACCOUNT_INDEX].account.owner = *program_id;
    keyed_accounts[TO_ACCOUNT_INDEX].account.data = vec![0; space as usize];
    keyed_accounts[TO_ACCOUNT_INDEX].account.executable = false;
    Ok(())
}

fn assign_account_to_program(
    keyed_accounts: &mut [KeyedAccount],
    program_id: &Pubkey,
) -> Result<(), SystemError> {
    keyed_accounts[FROM_ACCOUNT_INDEX].account.owner = *program_id;
    Ok(())
}
fn transfer_difs(
    keyed_accounts: &mut [KeyedAccount],
    difs: u64,
) -> Result<(), SystemError> {
    if difs > keyed_accounts[FROM_ACCOUNT_INDEX].account.difs {
        debug!(
            "Transfer: insufficient difs ({}, need {})",
            keyed_accounts[FROM_ACCOUNT_INDEX].account.difs, difs
        );
        Err(SystemError::ResultWithNegativeDifs)?;
    }
    keyed_accounts[FROM_ACCOUNT_INDEX].account.difs -= difs;
    keyed_accounts[TO_ACCOUNT_INDEX].account.difs += difs;
    Ok(())
}

fn transfer_reputations(
    keyed_accounts: &mut [KeyedAccount],
    reputations: u64,
) -> Result<(), SystemError> {
    if reputations > keyed_accounts[FROM_ACCOUNT_INDEX].account.reputations {
        debug!(
            "Transfer: insufficient reputations ({}, need {})",
            keyed_accounts[FROM_ACCOUNT_INDEX].account.reputations, reputations
        );
        Err(SystemError::ResultWithNegativeReputations)?;
    }
    keyed_accounts[FROM_ACCOUNT_INDEX].account.reputations -= reputations;
    keyed_accounts[TO_ACCOUNT_INDEX].account.reputations += reputations;
    Ok(())
}

pub fn process_instruction(
    _program_id: &Pubkey,
    keyed_accounts: &mut [KeyedAccount],
    data: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    if let Ok(instruction) = bincode::deserialize(data) {
        trace!("process_instruction: {:?}", instruction);
        trace!("keyed_accounts: {:?}", keyed_accounts);
        // All system instructions require that accounts_keys[0] be a signer
        if keyed_accounts[FROM_ACCOUNT_INDEX].signer_key().is_none() {
            debug!("account[from] is unsigned");
            Err(InstructionError::MissingRequiredSignature)?;
        }

        match instruction {
            SystemInstruction::CreateAccount {
                difs,
                reputations,
                space,
                program_id,
            } => create_system_account(keyed_accounts, difs, reputations, space, &program_id),
            SystemInstruction::CreateAccountWithReputation {
                reputations,
                space,
                program_id,
            } => create_system_account_with_reputation(keyed_accounts, reputations, space, &program_id),
            SystemInstruction::Assign { program_id } => {
                if !system_program::check_id(&keyed_accounts[FROM_ACCOUNT_INDEX].account.owner) {
                    Err(InstructionError::IncorrectProgramId)?;
                }
                assign_account_to_program(keyed_accounts, &program_id)
            }
            SystemInstruction::Transfer { difs } => transfer_difs(keyed_accounts, difs),
            SystemInstruction::TransferReputations { reputations } => transfer_reputations(keyed_accounts, reputations),
        }
        .map_err(|e| InstructionError::CustomError(e as u32))
    } else {
        debug!("Invalid instruction data: {:?}", data);
        Err(InstructionError::InvalidInstructionData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bank::Bank;
    use crate::bank_client::BankClient;
    use bincode::serialize;
    use morgan_interface::account::Account;
    use morgan_interface::client::SyncClient;
    use morgan_interface::genesis_block::create_genesis_block;
    use morgan_interface::instruction::{AccountMeta, Instruction, InstructionError};
    use morgan_interface::signature::{Keypair, KeypairUtil};
    use morgan_interface::system_program;
    use morgan_interface::transaction::TransactionError;

    #[test]
    fn test_create_system_account() {
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &system_program::id());

        let to = Pubkey::new_rand();
        let mut to_account = Account::new(0, 0, 0, &Pubkey::default());

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        create_system_account(&mut keyed_accounts, 50, 0, 2, &new_program_owner).unwrap();
        let from_difs = from_account.difs;
        let to_difs = to_account.difs;
        let to_reputations = to_account.reputations;
        let to_owner = to_account.owner;
        let to_data = to_account.data.clone();
        assert_eq!(from_difs, 50);
        assert_eq!(to_difs, 50);
        assert_eq!(to_reputations, 0);
        assert_eq!(to_owner, new_program_owner);
        assert_eq!(to_data, [0, 0]);
    }

    #[test]
    fn test_create_system_account_with_reputation() {
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(2, 100, 0, &system_program::id());

        let to = Pubkey::new_rand();
        let mut to_account = Account::new(0, 0, 0, &Pubkey::default());

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        create_system_account_with_reputation(&mut keyed_accounts, 50, 2, &new_program_owner).unwrap();
        let from_reputations = from_account.reputations;
        let to_reputations = to_account.reputations;
        let to_owner = to_account.owner;
        let to_data = to_account.data.clone();
        assert_eq!(from_reputations, 100);
        assert_eq!(to_reputations, 50);
        assert_eq!(to_owner, new_program_owner);
        assert_eq!(to_data, [0, 0]);
    }

    #[test]
    fn test_create_negative_difs() {
        // Attempt to create account with more difs than remaining in from_account
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &system_program::id());

        let to = Pubkey::new_rand();
        let mut to_account = Account::new(0, 0, 0, &Pubkey::default());
        let unchanged_account = to_account.clone();

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        let result = create_system_account(&mut keyed_accounts, 150, 0, 2, &new_program_owner);
        assert_eq!(result, Err(SystemError::ResultWithNegativeDifs));
        let from_difs = from_account.difs;
        assert_eq!(from_difs, 100);
        assert_eq!(to_account, unchanged_account);
    }

    #[test]
    fn test_create_negative_reputations() {
        // Attempt to create account with more reputations than remaining in from_account
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(0, 100, 0, &system_program::id());

        let to = Pubkey::new_rand();
        let mut to_account = Account::new(0, 0, 0, &Pubkey::default());
        let unchanged_account = to_account.clone();

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        let result = create_system_account_with_reputation(&mut keyed_accounts, 150, 2, &new_program_owner);
        assert_eq!(result, Err(SystemError::ResultWithNegativeDifs));
        let from_reputations = from_account.reputations;
        assert_eq!(from_reputations, 100);
        assert_eq!(to_account, unchanged_account);
    }


    #[test]
    fn test_create_already_owned() {
        // Attempt to create system account in account already owned by another program
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &system_program::id());

        let original_program_owner = Pubkey::new(&[5; 32]);
        let owned_key = Pubkey::new_rand();
        let mut owned_account = Account::new(0, 0, 0, &original_program_owner);
        let unchanged_account = owned_account.clone();

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&owned_key, false, &mut owned_account),
        ];
        let result = create_system_account(&mut keyed_accounts, 50, 0, 2, &new_program_owner);
        assert_eq!(result, Err(SystemError::AccountAlreadyInUse));
        let from_difs = from_account.difs;
        assert_eq!(from_difs, 100);
        assert_eq!(owned_account, unchanged_account);
    }

    #[test]
    fn test_create_data_populated() {
        // Attempt to create system account in account with populated data
        let new_program_owner = Pubkey::new(&[9; 32]);
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &system_program::id());

        let populated_key = Pubkey::new_rand();
        let mut populated_account = Account {
            difs: 0,
            reputations: 0,
            data: vec![0, 1, 2, 3],
            owner: Pubkey::default(),
            executable: false,
        };
        let unchanged_account = populated_account.clone();

        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&populated_key, false, &mut populated_account),
        ];
        let result = create_system_account(&mut keyed_accounts, 50, 0, 2, &new_program_owner);
        assert_eq!(result, Err(SystemError::AccountAlreadyInUse));
        assert_eq!(from_account.difs, 100);
        assert_eq!(populated_account, unchanged_account);
    }

    #[test]
    fn test_create_not_system_account() {
        let other_program = Pubkey::new(&[9; 32]);

        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &other_program);
        let to = Pubkey::new_rand();
        let mut to_account = Account::new(0, 0, 0, &Pubkey::default());
        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        let result = create_system_account(&mut keyed_accounts, 50, 0, 2, &other_program);
        assert_eq!(result, Err(SystemError::SourceNotSystemAccount));
    }

    #[test]
    fn test_assign_account_to_program() {
        let new_program_owner = Pubkey::new(&[9; 32]);

        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &system_program::id());
        let mut keyed_accounts = [KeyedAccount::new(&from, true, &mut from_account)];
        assign_account_to_program(&mut keyed_accounts, &new_program_owner).unwrap();
        let from_owner = from_account.owner;
        assert_eq!(from_owner, new_program_owner);

        // Attempt to assign account not owned by system program
        let another_program_owner = Pubkey::new(&[8; 32]);
        keyed_accounts = [KeyedAccount::new(&from, true, &mut from_account)];
        let instruction = SystemInstruction::Assign {
            program_id: another_program_owner,
        };
        let data = serialize(&instruction).unwrap();
        let result = process_instruction(&system_program::id(), &mut keyed_accounts, &data, 0);
        assert_eq!(result, Err(InstructionError::IncorrectProgramId));
        assert_eq!(from_account.owner, new_program_owner);
    }

    #[test]
    fn test_transfer_difs() {
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 0, 0, &Pubkey::new(&[2; 32])); // account owner should not matter
        let to = Pubkey::new_rand();
        let mut to_account = Account::new(1, 0, 0, &Pubkey::new(&[3; 32])); // account owner should not matter
        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        transfer_difs(&mut keyed_accounts, 50).unwrap();
        let from_difs = from_account.difs;
        let to_difs = to_account.difs;
        assert_eq!(from_difs, 50);
        assert_eq!(to_difs, 51);

        // Attempt to move more difs than remaining in from_account
        keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        let result = transfer_difs(&mut keyed_accounts, 100);
        assert_eq!(result, Err(SystemError::ResultWithNegativeDifs));
        assert_eq!(from_account.difs, 50);
        assert_eq!(to_account.difs, 51);
    }

    #[test]
    fn test_transfer_reputations() {
        let from = Pubkey::new_rand();
        let mut from_account = Account::new(100, 100, 0, &Pubkey::new(&[2; 32])); // account owner should not matter
        let to = Pubkey::new_rand();
        let mut to_account = Account::new(1, 0, 0, &Pubkey::new(&[3; 32])); // account owner should not matter
        let mut keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        transfer_reputations(&mut keyed_accounts, 50).unwrap();
        let from_reputations = from_account.reputations;
        let to_reputations = to_account.reputations;
        assert_eq!(from_reputations, 50);
        assert_eq!(to_reputations, 50);

        // Attempt to move more difs than remaining in from_account
        keyed_accounts = [
            KeyedAccount::new(&from, true, &mut from_account),
            KeyedAccount::new(&to, false, &mut to_account),
        ];
        let result = transfer_reputations(&mut keyed_accounts, 100);
        assert_eq!(result, Err(SystemError::ResultWithNegativeReputations));
        assert_eq!(from_account.difs, 100);
        assert_eq!(to_account.difs, 1);
    }

    #[test]
    fn test_system_unsigned_transaction() {
        let (genesis_block, alice_keypair) = create_genesis_block(100);
        let alice_pubkey = alice_keypair.pubkey();
        let mallory_keypair = Keypair::new();
        let mallory_pubkey = mallory_keypair.pubkey();

        // Fund to account to bypass AccountNotFound error
        let bank = Bank::new(&genesis_block);
        let bank_client = BankClient::new(bank);
        bank_client
            .transfer(50, &alice_keypair, &mallory_pubkey)
            .unwrap();

        // Erroneously sign transaction with recipient account key
        // No signature case is tested by bank `test_zero_signatures()`
        let account_metas = vec![
            AccountMeta::new(alice_pubkey, false),
            AccountMeta::new(mallory_pubkey, true),
        ];
        let malicious_instruction = Instruction::new(
            system_program::id(),
            &SystemInstruction::Transfer { difs: 10 },
            account_metas,
        );
        assert_eq!(
            bank_client
                .send_instruction(&mallory_keypair, malicious_instruction)
                .unwrap_err()
                .unwrap(),
            TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature)
        );
        assert_eq!(bank_client.get_balance(&alice_pubkey).unwrap(), 50);
        assert_eq!(bank_client.get_balance(&mallory_pubkey).unwrap(), 50);
    }
}
