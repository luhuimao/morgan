use crate::morgan_storage_controller;
use morgan_interface::genesis_block::GenesisBlock;
use morgan_interface::pubkey::Pubkey;
use morgan_storage_api::storage_contract;

pub trait GenesisBlockUtil {
    fn add_storage_controller(&mut self, validator_storage_pubkey: &Pubkey);
}

impl GenesisBlockUtil for GenesisBlock {
    fn add_storage_controller(&mut self, validator_storage_pubkey: &Pubkey) {
        self.accounts.push((
            *validator_storage_pubkey,
            storage_contract::create_validator_storage_account(1),
        ));
        self.native_instruction_processors
            .push(morgan_storage_controller!());
    }
}
