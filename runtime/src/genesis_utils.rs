use morgan_interface::account::Account;
use morgan_interface::genesis_block::GenesisBlock;
use morgan_interface::pubkey::Pubkey;
use morgan_interface::signature::{Keypair, KeypairUtil};
use morgan_interface::system_program;
use morgan_stake_api::stake_state;
use morgan_vote_api::vote_state;

// The default stake placed with the bootstrap leader
pub const BOOTSTRAP_LEADER_DIFS: u64 = 42;

pub struct GenesisBlockInfo {
    pub genesis_block: GenesisBlock,
    pub mint_keypair: Keypair,
    pub voting_keypair: Keypair,
}

pub fn create_genesis_block_with_leader(
    mint_difs: u64,
    bootstrap_leader_pubkey: &Pubkey,
    bootstrap_leader_stake_difs: u64,
) -> GenesisBlockInfo {
    let mint_keypair = Keypair::new();
    let voting_keypair = Keypair::new();
    let staking_keypair = Keypair::new();

    // TODO: de-duplicate the stake once passive staking
    //  is fully implemented
    let (vote_account, vote_state) = vote_state::create_bootstrap_leader_account(
        &voting_keypair.pubkey(),
        &bootstrap_leader_pubkey,
        0,
        bootstrap_leader_stake_difs,
    );

    let genesis_block = GenesisBlock::new(
        &bootstrap_leader_pubkey,
        &[
            // the mint
            (
                mint_keypair.pubkey(),
                Account::new(mint_difs, 0, 0, &system_program::id()),
            ),
            // node needs an account to issue votes and storage proofs from, this will require
            //  airdrops at some point to cover fees...
            (
                *bootstrap_leader_pubkey,
                Account::new(42, 0, 0, &system_program::id()),
            ),
            // where votes go to
            (voting_keypair.pubkey(), vote_account),
            // passive bootstrap leader stake, duplicates above temporarily
            (
                staking_keypair.pubkey(),
                stake_state::create_delegate_stake_account(
                    &voting_keypair.pubkey(),
                    &vote_state,
                    bootstrap_leader_stake_difs,
                ),
            ),
        ],
        &[morgan_vote_controller!(), morgan_vote_controller!()],
    );

    GenesisBlockInfo {
        genesis_block,
        mint_keypair,
        voting_keypair,
    }
}
