use morgan_runtime::bank::Bank;
use morgan_runtime::bank_client::BankClient;
use morgan_runtime::loader_utils::{create_invoke_instruction, load_program};
use morgan_sdk::client::SyncClient;
use morgan_sdk::genesis_block::create_genesis_block;
use morgan_sdk::native_loader;
use morgan_sdk::signature::KeypairUtil;

#[test]
fn test_program_native_noop() {
    morgan_logger::setup();

    let (genesis_block, alice_keypair) = create_genesis_block(50);
    let bank = Bank::new(&genesis_block);
    let bank_client = BankClient::new(bank);

    let program = "morgan_noop_program".as_bytes().to_vec();
    let program_id = load_program(&bank_client, &alice_keypair, &native_loader::id(), program);

    // Call user program
    let instruction = create_invoke_instruction(alice_keypair.pubkey(), program_id, &1u8);
    bank_client
        .send_instruction(&alice_keypair, instruction)
        .unwrap();
}
