//! Stakes serve as a cache of stake and vote accounts to derive
//! node stakes
use hashbrown::HashMap;
use morgan_sdk::account::Account;
use morgan_sdk::pubkey::Pubkey;
use morgan_stake_api::stake_state::StakeState;

#[derive(Default, Clone)]
pub struct Stakes {
    /// vote accounts
    vote_accounts: HashMap<Pubkey, (u64, Account)>,

    /// stake_accounts
    stake_accounts: HashMap<Pubkey, Account>,
}

impl Stakes {
    // sum the stakes that point to the given voter_pubkey
    fn calculate_stake(&self, voter_pubkey: &Pubkey) -> u64 {
        self.stake_accounts
            .iter()
            .filter(|(_, stake_account)| {
                Some(*voter_pubkey) == StakeState::voter_pubkey_from(stake_account)
            })
            .map(|(_, stake_account)| stake_account.difs)
            .sum()
    }

    pub fn is_stake(account: &Account) -> bool {
        morgan_vote_api::check_id(&account.owner) || morgan_stake_api::check_id(&account.owner)
    }

    pub fn store(&mut self, pubkey: &Pubkey, account: &Account) {
        if morgan_vote_api::check_id(&account.owner) {
            if account.difs == 0 {
                self.vote_accounts.remove(pubkey);
            } else {
                // update the stake of this entry
                let stake = self
                    .vote_accounts
                    .get(pubkey)
                    .map_or_else(|| self.calculate_stake(pubkey), |v| v.0);

                self.vote_accounts.insert(*pubkey, (stake, account.clone()));
            }
        } else if morgan_stake_api::check_id(&account.owner) {
            //  old_stake is stake difs and voter_pubkey from the pre-store() version
            let old_stake = self.stake_accounts.get(pubkey).and_then(|old_account| {
                StakeState::voter_pubkey_from(old_account)
                    .map(|old_voter_pubkey| (old_account.difs, old_voter_pubkey))
            });

            let stake = StakeState::voter_pubkey_from(account)
                .map(|voter_pubkey| (account.difs, voter_pubkey));

            // if adjustments need to be made...
            if stake != old_stake {
                if let Some((old_stake, old_voter_pubkey)) = old_stake {
                    self.vote_accounts
                        .entry(old_voter_pubkey)
                        .and_modify(|e| e.0 -= old_stake);
                }
                if let Some((stake, voter_pubkey)) = stake {
                    self.vote_accounts
                        .entry(voter_pubkey)
                        .and_modify(|e| e.0 += stake);
                }
            }

            if account.difs == 0 {
                self.stake_accounts.remove(pubkey);
            } else {
                self.stake_accounts.insert(*pubkey, account.clone());
            }
        }
    }
    pub fn vote_accounts(&self) -> &HashMap<Pubkey, (u64, Account)> {
        &self.vote_accounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morgan_sdk::pubkey::Pubkey;
    use morgan_stake_api::stake_state;
    use morgan_vote_api::vote_state::{self, VoteState};

    //  set up some dummies  for a staked node    ((     vote      )  (     stake     ))
    fn create_staked_node_accounts(stake: u64) -> ((Pubkey, Account), (Pubkey, Account)) {
        let vote_pubkey = Pubkey::new_rand();
        let vote_account = vote_state::create_account(&vote_pubkey, &Pubkey::new_rand(), 0, 1);
        (
            (vote_pubkey, vote_account),
            create_stake_account(stake, &vote_pubkey),
        )
    }

    //   add stake to a vote_pubkey                               (   stake    )
    fn create_stake_account(stake: u64, vote_pubkey: &Pubkey) -> (Pubkey, Account) {
        (
            Pubkey::new_rand(),
            stake_state::create_delegate_stake_account(&vote_pubkey, &VoteState::default(), stake),
        )
    }

    #[test]
    fn test_stakes_basic() {
        let mut stakes = Stakes::default();

        let ((vote_pubkey, vote_account), (stake_pubkey, mut stake_account)) =
            create_staked_node_accounts(10);

        stakes.store(&vote_pubkey, &vote_account);
        stakes.store(&stake_pubkey, &stake_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 10);
        }

        stake_account.difs = 42;
        stake_account.difs1 = 42;
        stakes.store(&stake_pubkey, &stake_account);
        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 42);
        }

        stake_account.difs = 0;
        stake_account.difs1 = 0;
        stakes.store(&stake_pubkey, &stake_account);
        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 0);
        }
    }

    #[test]
    fn test_stakes_vote_account_disappear_reappear() {
        let mut stakes = Stakes::default();

        let ((vote_pubkey, mut vote_account), (stake_pubkey, stake_account)) =
            create_staked_node_accounts(10);

        stakes.store(&vote_pubkey, &vote_account);
        stakes.store(&stake_pubkey, &stake_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 10);
        }

        vote_account.difs = 0;
        vote_account.difs1 = 0;
        stakes.store(&vote_pubkey, &vote_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_none());
        }
        vote_account.difs = 1;
        vote_account.difs1 = 1;
        stakes.store(&vote_pubkey, &vote_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 10);
        }
    }

    #[test]
    fn test_stakes_change_delegate() {
        let mut stakes = Stakes::default();

        let ((vote_pubkey, vote_account), (stake_pubkey, stake_account)) =
            create_staked_node_accounts(10);

        let ((vote_pubkey2, vote_account2), (_stake_pubkey2, stake_account2)) =
            create_staked_node_accounts(10);

        stakes.store(&vote_pubkey, &vote_account);
        stakes.store(&vote_pubkey2, &vote_account2);

        // delegates to vote_pubkey
        stakes.store(&stake_pubkey, &stake_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 10);
            assert!(vote_accounts.get(&vote_pubkey2).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey2).unwrap().0, 0);
        }

        // delegates to vote_pubkey2
        stakes.store(&stake_pubkey, &stake_account2);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 0);
            assert!(vote_accounts.get(&vote_pubkey2).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey2).unwrap().0, 10);
        }
    }
    #[test]
    fn test_stakes_multiple_stakers() {
        let mut stakes = Stakes::default();

        let ((vote_pubkey, vote_account), (stake_pubkey, stake_account)) =
            create_staked_node_accounts(10);

        let (stake_pubkey2, stake_account2) = create_stake_account(10, &vote_pubkey);

        stakes.store(&vote_pubkey, &vote_account);

        // delegates to vote_pubkey
        stakes.store(&stake_pubkey, &stake_account);
        stakes.store(&stake_pubkey2, &stake_account2);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 20);
        }
    }

    #[test]
    fn test_stakes_not_delegate() {
        let mut stakes = Stakes::default();

        let ((vote_pubkey, vote_account), (stake_pubkey, stake_account)) =
            create_staked_node_accounts(10);

        stakes.store(&vote_pubkey, &vote_account);
        stakes.store(&stake_pubkey, &stake_account);

        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 10);
        }

        // not a stake account, and whacks above entry
        stakes.store(&stake_pubkey, &Account::new(1, 0, &morgan_stake_api::id()));
        {
            let vote_accounts = stakes.vote_accounts();
            assert!(vote_accounts.get(&vote_pubkey).is_some());
            assert_eq!(vote_accounts.get(&vote_pubkey).unwrap().0, 0);
        }
    }

}
