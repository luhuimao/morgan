use morgan_runtime::bank::Bank;
use morgan_runtime::bank_client::BankClient;
use morgan_runtime::loader_utils::{create_invoke_instruction, load_program};
use morgan_interface::client::SyncClient;
use morgan_interface::genesis_block::create_genesis_block;
use morgan_interface::instruction::InstructionError;
use morgan_interface::native_loader;
use morgan_interface::signature::KeypairUtil;
use morgan_interface::transaction::TransactionError;

#[test]
fn test_program_native_failure() {
    let (genesis_block, alice_keypair) = create_genesis_block(50);
    let bank = Bank::new(&genesis_block);
    let bank_client = BankClient::new(bank);

    let program = "morgan_failure_program".as_bytes().to_vec();
    let program_id = load_program(&bank_client, &alice_keypair, &native_loader::id(), program);

    // Call user program
    let instruction = create_invoke_instruction(alice_keypair.pubkey(), program_id, &1u8);
    assert_eq!(
        bank_client
            .send_instruction(&alice_keypair, instruction)
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::GenericError)
    );
}
