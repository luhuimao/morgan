//! Config program

use log::*;
use morgan_interface::account::KeyedAccount;
use morgan_interface::instruction::InstructionError;
use morgan_interface::pubkey::Pubkey;
use morgan_helper::logHelper::*;

pub fn process_instruction(
    _program_id: &Pubkey,
    keyed_accounts: &mut [KeyedAccount],
    data: &[u8],
    _tick_height: u64,
) -> Result<(), InstructionError> {
    if keyed_accounts[0].signer_key().is_none() {
        // error!("{}", Error(format!("account[0].signer_key().is_none()").to_string()));
        println!(
            "{}",
            Error(
                format!("account[0].signer_key().is_none()").to_string(),
                module_path!().to_string()
            )
        );
        Err(InstructionError::MissingRequiredSignature)?;
    }

    if keyed_accounts[0].account.data.len() < data.len() {
        // error!("{}", Error(format!("instruction data too large").to_string()));
        println!(
            "{}",
            Error(
                format!("instruction data too large").to_string(),
                module_path!().to_string()
            )
        );
        Err(InstructionError::InvalidInstructionData)?;
    }

    keyed_accounts[0].account.data[0..data.len()].copy_from_slice(data);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config_instruction, id, ConfigState};
    use bincode::{deserialize, serialized_size};
    use serde_derive::{Deserialize, Serialize};
    use morgan_runtime::bank::Bank;
    use morgan_runtime::bank_client::BankClient;
    use morgan_interface::client::SyncClient;
    use morgan_interface::genesis_block::create_genesis_block;
    use morgan_interface::message::Message;
    use morgan_interface::signature::{Keypair, KeypairUtil};
    use morgan_interface::system_instruction;

    #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
    struct MyConfig {
        pub item: u64,
    }
    impl MyConfig {
        pub fn new(item: u64) -> Self {
            Self { item }
        }
        pub fn deserialize(input: &[u8]) -> Option<Self> {
            deserialize(input).ok()
        }
    }

    impl ConfigState for MyConfig {
        fn max_space() -> u64 {
            serialized_size(&Self::default()).unwrap()
        }
    }

    fn create_bank(difs: u64) -> (Bank, Keypair) {
        let (genesis_block, mint_keypair) = create_genesis_block(difs);
        let mut bank = Bank::new(&genesis_block);
        bank.add_instruction_processor(id(), process_instruction);
        (bank, mint_keypair)
    }

    fn create_config_account(bank: Bank, mint_keypair: &Keypair) -> (BankClient, Keypair) {
        let config_keypair = Keypair::new();
        let config_pubkey = config_keypair.pubkey();

        let bank_client = BankClient::new(bank);
        bank_client
            .send_instruction(
                mint_keypair,
                config_instruction::create_account::<MyConfig>(
                    &mint_keypair.pubkey(),
                    &config_pubkey,
                    1,
                ),
            )
            .expect("new_account");

        (bank_client, config_keypair)
    }

    #[test]
    fn test_process_create_ok() {
        morgan_logger::setup();
        let (bank, mint_keypair) = create_bank(10_000);
        let (bank_client, config_keypair) = create_config_account(bank, &mint_keypair);
        let config_account_data = bank_client
            .get_account_data(&config_keypair.pubkey())
            .unwrap()
            .unwrap();
        assert_eq!(
            MyConfig::default(),
            MyConfig::deserialize(&config_account_data).unwrap()
        );
    }

    #[test]
    fn test_process_store_ok() {
        morgan_logger::setup();
        let (bank, mint_keypair) = create_bank(10_000);
        let (bank_client, config_keypair) = create_config_account(bank, &mint_keypair);
        let config_pubkey = config_keypair.pubkey();

        let my_config = MyConfig::new(42);

        let instruction = config_instruction::store(&config_pubkey, &my_config);
        let message = Message::new_with_payer(vec![instruction], Some(&mint_keypair.pubkey()));
        bank_client
            .send_message(&[&mint_keypair, &config_keypair], message)
            .unwrap();

        let config_account_data = bank_client
            .get_account_data(&config_pubkey)
            .unwrap()
            .unwrap();
        assert_eq!(
            my_config,
            MyConfig::deserialize(&config_account_data).unwrap()
        );
    }

    #[test]
    fn test_process_store_fail_instruction_data_too_large() {
        morgan_logger::setup();
        let (bank, mint_keypair) = create_bank(10_000);
        let (bank_client, config_keypair) = create_config_account(bank, &mint_keypair);
        let config_pubkey = config_keypair.pubkey();

        let my_config = MyConfig::new(42);

        let mut instruction = config_instruction::store(&config_pubkey, &my_config);
        instruction.data = vec![0; 123]; // <-- Replace data with a vector that's too large
        let message = Message::new(vec![instruction]);
        bank_client
            .send_message(&[&config_keypair], message)
            .unwrap_err();
    }

    #[test]
    fn test_process_store_fail_account0_not_signer() {
        morgan_logger::setup();
        let (bank, mint_keypair) = create_bank(10_000);
        let system_keypair = Keypair::new();
        let system_pubkey = system_keypair.pubkey();

        bank.transfer(42, &mint_keypair, &system_pubkey).unwrap();
        let (bank_client, config_keypair) = create_config_account(bank, &mint_keypair);
        let config_pubkey = config_keypair.pubkey();

        let transfer_instruction =
            system_instruction::transfer(&system_pubkey, &Pubkey::new_rand(), 42);
        let my_config = MyConfig::new(42);
        let mut store_instruction = config_instruction::store(&config_pubkey, &my_config);
        store_instruction.accounts[0].is_signer = false; // <----- not a signer

        let message = Message::new(vec![transfer_instruction, store_instruction]);
        bank_client
            .send_message(&[&system_keypair], message)
            .unwrap_err();
    }
}
