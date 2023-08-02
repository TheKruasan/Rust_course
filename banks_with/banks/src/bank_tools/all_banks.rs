use crate::bank_tools::{bank::Bank, bank_account::BankAccount};
use lazy_static;
use std::{collections::HashMap, sync::{RwLock, Arc}};

lazy_static! {
    // Define a [`HashMap`] with all banks 
    // pub static ref BANKS: Arc<RwLock<HashMap<u32, Bank>>> = {
    //     let map = HashMap::new();
    //     Arc::new(RwLock::new(map))
    // };
    #[derive(Debug)]
    pub static ref BANKS: Arc<HashMap<u32, Bank>> = {
        let mut map = HashMap::new();
        let mut mock_accounts = HashMap::new();
        mock_accounts.insert(
            1,
            BankAccount {
                id: 1,
                balance: 1000,
            },
        );
        mock_accounts.insert(
            2,
            BankAccount {
                id: 2,
                balance: 1000,
            },
        );
        (map).insert(1,(Bank::new(1, 100, mock_accounts.clone())));
        (map).insert(2,(Bank::new(2, 100, mock_accounts.clone())));
        Arc::new(map)
    };
}
