use crate::bank_tools::{all_banks::BANKS, bank_account::BankAccount, transaction::Transaction};
use std::{collections::HashMap, sync::{mpsc, RwLock, Arc}, thread, sync::Weak};

#[derive(Debug)]
pub struct Bank {
    pub id: u32,
    pub balance: u32,
    pub accounts: HashMap<u32, BankAccount>,
    pub thread: thread::JoinHandle<()>,
    sender: mpsc::SyncSender<Transaction>,
}

impl Bank {
    /// Create a [`Bank`] instance from id, balance and accounts given in params.
    /// Create a thread inside, initialize sender, and give reciever to thread
    pub fn new(id: u32, balance: u32, accounts: HashMap<u32, BankAccount>) -> Bank {
        let (tx, rx) = mpsc::sync_channel(0);


        let thread = thread::spawn(move || loop {
            // get a message
            let message = rx.recv(); // block until get a message

            // check message
            match message {
                Ok(transaction) => {
                    println!("Bank {id} got a transaction {:#?}; executing.", transaction);
                    
                    Bank::handle_transaction(Arc::downgrade(&BANKS), id, transaction);
                }
                Err(_) => {
                    println!("Bank {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        // return bank
        Bank {
            id,
            balance,
            accounts,
            thread,
            sender: tx,
        }
    }

    /// Handle a transaction given in params
    /// Change balance of accounts in one bank or in differnt banks
    /// Bank change account balance if only it is the target bank in transaction
    fn handle_transaction(banks: Weak<RwLock<HashMap<u32, Bank>>>, bank_id: u32, transaction: Transaction) {
        // get HashMap of all banks
        println!("start to handle transaction");
        if let Some(all_banks) = banks.upgrade() {
            let mut bank_from = all_banks.write().unwrap();
            let bank_from_1 = bank_from.get_mut(&bank_id).unwrap();
            println!("all banks");
            // get bank and account where value will be substrued            
            println!("find nessesary bank");
            let mut account_from = bank_from_1.accounts.get_mut(&transaction.account_id).unwrap();
            println!("find nessesary account");
            // if it is internal transaction, add value to another account in this bank
            if transaction.target_bank_id == bank_id {
                // substruct value
                account_from.substruct_from_balance(transaction.value);
                // get target account
                let account_to = bank_from_1
                    .accounts
                    .get_mut(&transaction.target_account_id)
                    .unwrap();
                account_to.add_to_balance(transaction.value);
            }
            // else send this transaction to another bank
            else {
                let bank_to = all_banks.read().unwrap();
                let bank_to_1 = bank_to.get(&transaction.target_bank_id).unwrap();
                bank_to_1.send_transaction(transaction);
            }

        }

        // let all_banks = BANKS.read().unwrap();
        // println!("{:#?}", all_banks);
    }
    
    /// Sends transaction to bank given in params
    pub fn send_transaction(&self, transaction: Transaction) {
        self.sender.send(transaction).unwrap();
    }
}