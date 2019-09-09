use morgan_sdk::hash::Hash;
use morgan_sdk::pubkey::Pubkey;
use morgan_sdk::signature::{Keypair, KeypairUtil};
use morgan_sdk::system_transaction;
use morgan_sdk::transaction::Transaction;
use morgan_drone::drone::AirdropValueType;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;

pub fn request_airdrop_transaction(
    _drone_addr: &SocketAddr,
    _id: &Pubkey,
    value: u64,
    _blockhash: Hash,
    value_type: AirdropValueType;
) -> Result<Transaction, Error> {
    if value == 0 {
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))?
    }
    let key = Keypair::new();
    let to = Pubkey::new_rand();
    let blockhash = Hash::default();
    let tx = if value_type == AirdropValueType::Difs {
        system_transaction::create_user_account(&key, &to, value, blockhash)
    } else {
        system_transaction::create_user_account_with_difs1(&key, &to, value, blockhash)
    }
    Ok(tx)
}
