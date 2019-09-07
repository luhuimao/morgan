use morgan_drone::drone::{request_airdrop_transaction, run_local_drone};
use morgan_sdk::hash::Hash;
use morgan_sdk::message::Message;
use morgan_sdk::pubkey::Pubkey;
use morgan_sdk::signature::{Keypair, KeypairUtil};
use morgan_sdk::system_instruction;
use morgan_sdk::transaction::Transaction;
use std::sync::mpsc::channel;

#[test]
fn test_local_drone() {
    let keypair = Keypair::new();
    let to = Pubkey::new_rand();
    let difs = 50;
    let blockhash = Hash::new(&to.as_ref());
    let create_instruction =
        system_instruction::create_user_account(&keypair.pubkey(), &to, difs);
    let message = Message::new(vec![create_instruction]);
    let expected_tx = Transaction::new(&[&keypair], message, blockhash);

    let (sender, receiver) = channel();
    run_local_drone(keypair, sender, None);
    let drone_addr = receiver.recv().unwrap();

    let result = request_airdrop_transaction(&drone_addr, &to, difs, blockhash, AirdropValueType::Difs);
    assert_eq!(expected_tx, result.unwrap());
}
