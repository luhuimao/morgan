use crate::pubkey::Pubkey;
use std::{cmp, fmt};

/// An Account with data that is stored on chain
#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct Account {
    /// difs in the account
    pub difs: u64,
    /// data held in this account
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// test field for future reputation value
    pub difs1: u64,
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_len = cmp::min(64, self.data.len());
        let data_str = if data_len > 0 {
            format!(" data: {}", hex::encode(self.data[..data_len].to_vec()))
        } else {
            "".to_string()
        };
        write!(
            f,
            "Account {{ difs: {} data.len: {} owner: {} executable: {}{} }}",
            self.difs,
            self.data.len(),
            self.owner,
            self.executable,
            data_str,
        )
    }
}

impl Account {
    // TODO do we want to add executable and leader_owner even though they should always be false/default?
    pub fn new(difs: u64, space: usize, owner: &Pubkey) -> Account {
        Account {
            difs,
            data: vec![0u8; space],
            owner: *owner,
            executable: false,
            difs,
        }
    }

    pub fn deserialize_data<T: serde::de::DeserializeOwned>(&self) -> Result<T, bincode::Error> {
        bincode::deserialize(&self.data)
    }

    pub fn serialize_data<T: serde::Serialize>(&mut self, state: &T) -> Result<(), bincode::Error> {
        bincode::serialize_into(&mut self.data[..], state)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct KeyedAccount<'a> {
    is_signer: bool, // Transaction was signed by this account's key
    key: &'a Pubkey,
    pub account: &'a mut Account,
}

impl<'a> KeyedAccount<'a> {
    pub fn signer_key(&self) -> Option<&Pubkey> {
        if self.is_signer {
            Some(self.key)
        } else {
            None
        }
    }

    pub fn unsigned_key(&self) -> &Pubkey {
        self.key
    }

    pub fn new(key: &'a Pubkey, is_signer: bool, account: &'a mut Account) -> KeyedAccount<'a> {
        KeyedAccount {
            key,
            is_signer,
            account,
        }
    }
}

impl<'a> From<(&'a Pubkey, &'a mut Account)> for KeyedAccount<'a> {
    fn from((key, account): (&'a Pubkey, &'a mut Account)) -> Self {
        KeyedAccount {
            is_signer: false,
            key,
            account,
        }
    }
}

impl<'a> From<&'a mut (Pubkey, Account)> for KeyedAccount<'a> {
    fn from((key, account): &'a mut (Pubkey, Account)) -> Self {
        KeyedAccount {
            is_signer: false,
            key,
            account,
        }
    }
}

pub fn create_keyed_accounts(accounts: &mut [(Pubkey, Account)]) -> Vec<KeyedAccount> {
    accounts.iter_mut().map(Into::into).collect()
}
