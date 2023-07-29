use std::{collections::HashMap, thread};

use banks::{Bank, TransferBetweenAccounts, BANKS, BankAccount};

fn main() {

    let requests = vec![(
        1,
        TransferBetweenAccounts {
            account_id: 1,
            target_bank_id: 2,
            target_account_id: 2,
            value: 12,
        },
    )];
    let mut accounts = HashMap::new();
    accounts.insert(1, BankAccount{
        id: 1,
        balance: 100
    });
    accounts.insert(2, BankAccount{
        id: 2,
        balance: 100
    });
    BANKS.write().unwrap().insert(
        1,
        Bank {
            id: 1,
            balance: 20000,
            accounts: accounts.clone(),
        },
    );
    BANKS.write().unwrap().insert(
        2,
        Bank {
            id: 2,
            balance: 20000,
            accounts: accounts.clone(),
        },
    );


    
    for request in requests {
        BANKS
            .write()
            .expect("error in main")
            .get_mut(&request.0)
            .expect("No bank")
            .add_request(request.1);
    }

    println!("{:#?}", BANKS.read().unwrap());
}