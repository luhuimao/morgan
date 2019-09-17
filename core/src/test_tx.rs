use morgan_interface::hash::Hash;
use morgan_interface::instruction::CompiledInstruction;
use morgan_interface::signature::{Keypair, KeypairUtil};
use morgan_interface::system_instruction::SystemInstruction;
use morgan_interface::system_program;
use morgan_interface::system_transaction;
use morgan_interface::transaction::Transaction;

pub fn test_tx() -> Transaction {
    let keypair1 = Keypair::new();
    let pubkey1 = keypair1.pubkey();
    let zero = Hash::default();
    system_transaction::create_user_account(&keypair1, &pubkey1, 42, zero)
}

pub fn test_multisig_tx() -> Transaction {
    let keypair0 = Keypair::new();
    let keypair1 = Keypair::new();
    let keypairs = vec![&keypair0, &keypair1];
    let difs = 5;
    let blockhash = Hash::default();

    let transfer_instruction = SystemInstruction::Transfer { difs };

    let program_ids = vec![system_program::id(), morgan_budget_api::id()];

    let instructions = vec![CompiledInstruction::new(
        0,
        &transfer_instruction,
        vec![0, 1],
    )];

    Transaction::new_with_compiled_instructions(
        &keypairs,
        &[],
        blockhash,
        program_ids,
        instructions,
    )
}
