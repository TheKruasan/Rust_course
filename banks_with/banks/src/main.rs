use std::{collections::HashMap, thread, time::Duration};

use banks::bank_tools::{
    all_banks::BANKS, bank::Bank, bank_account::BankAccount, transaction::Transaction,
};

fn main() {
    // creating a mock data ===========================
    create_mock_data();
    let mock_transaction = Transaction {
        account_id: 1,
        target_bank_id: 2,
        target_account_id: 1,
        value: 123,
    };
    let all_banks = BANKS.read().unwrap();
    // =================================

    // send a transaction
    let bank_from = all_banks.get(&1).unwrap();
    println!("{:?}",bank_from);
    bank_from.send_transaction(mock_transaction);
    // wait for result
    thread::sleep(Duration::from_secs(4));
    // check result
    println!("{:#?}", BANKS.read().unwrap());
}
// creates a mock data for tests
fn create_mock_data(){
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
    BANKS.write().unwrap().insert(1, Bank::new(1, 100, mock_accounts.clone()));
    BANKS.write().unwrap().insert(2, Bank::new(2, 100, mock_accounts.clone()));
}