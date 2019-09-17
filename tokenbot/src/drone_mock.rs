use morgan_interface::hash::Hash;
use morgan_interface::pubkey::Pubkey;
use morgan_interface::signature::{Keypair, KeypairUtil};
use morgan_interface::system_transaction;
use morgan_interface::transaction::Transaction;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;

pub fn request_airdrop_transaction(
    _drone_addr: &SocketAddr,
    _id: &Pubkey,
    difs: u64,
    _blockhash: Hash,
) -> Result<Transaction, Error> {
    if difs == 0 {
        Err(Error::new(ErrorKind::Other, "Airdrop failed"))?
    }
    let key = Keypair::new();
    let to = Pubkey::new_rand();
    let blockhash = Hash::default();
    let tx = system_transaction::create_user_account(&key, &to, difs, blockhash);
    Ok(tx)
}
