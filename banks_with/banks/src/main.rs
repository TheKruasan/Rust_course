// use std::{collections::HashMap, thread, time::Duration};

// use banks::bank_tools::{
//     bank::Bank, bank_account::BankAccount, transaction::Transaction,
// };

// use std::{sync::{mpsc, RwLock, Arc}};


// fn main() {
//     // creating a mock data ===========================
//     let all_banks = create_mock_data();
//     let mock_transaction = Transaction {
//         account_id: 1,
//         target_bank_id: 2,
//         target_account_id: 1,
//         value: 123,
//     };
    
//     // send a transaction
//     let all_banks_1 = all_banks.read().unwrap();
//     let bank_from = {
//         all_banks_1.get(&1).unwrap().clone()
//     };
//     println!("{:?}",bank_from);
//     bank_from.send_transaction(mock_transaction);
//     // wait for result
//     thread::sleep(Duration::from_secs(2));
//     // check result
//     println!("{:#?}", all_banks.read().unwrap());
// }

// // creates a mock data for tests
// fn create_mock_data() -> Arc<RwLock<HashMap<u32, Bank>>> {
//     let mut mock_accounts = HashMap::new();

//     mock_accounts.insert(
//         1,
//         BankAccount {
//             id: 1,
//             balance: 1000,
//         },
//     );
//     mock_accounts.insert(
//         2,
//         BankAccount {
//             id: 2,
//             balance: 1000,
//         },
//     );

//     let map = HashMap::new();
//     let banks = Arc::new(RwLock::new(map));
//     banks.write().unwrap().insert(1, Bank::new( 1, 100,mock_accounts.clone()));
//     banks.write().unwrap().insert(2, Bank::new( 2, 100 ,mock_accounts.clone()));
//     banks
// }



use std::{collections::HashMap, thread, time::Duration, sync::RwLock};

use banks::bank_tools::{
    all_banks::BANKS, bank::Bank, bank_account::BankAccount, transaction::Transaction,
};

fn main() { 
    // creating a mock data ===========================
    for (k,v) in BANKS.iter(){
        println!("{:?}",v.accounts.read().unwrap());
    }
    let mock_transaction_1 = Transaction {
        bank_from:1,
        account_id: 1, 
        target_bank_id: 2,
        target_account_id: 1,
        value: 123,
    };
    let mock_transaction_2 = Transaction {
        bank_from:2,
        account_id: 2, 
        target_bank_id: 2,
        target_account_id: 1,
        value: 300,
    };
    // =================================

    // send a transaction
    let bank_from_1 = BANKS.get(&mock_transaction_1.bank_from).unwrap();
    let bank_from_2 = BANKS.get(&mock_transaction_2.bank_from).unwrap();

    bank_from_1.send_transaction(mock_transaction_1);
    bank_from_2.send_transaction(mock_transaction_2);
    // wait for result
    thread::sleep(Duration::from_secs(4));
    // check result
    for (k,v) in BANKS.iter(){
        println!("{} is {:?}",v.id,v.accounts.read().unwrap());
    }
}