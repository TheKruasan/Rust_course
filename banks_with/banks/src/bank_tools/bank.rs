use crate::bank_tools::{all_banks::BANKS, bank_account::BankAccount, transaction::Transaction, bank};
use std::{collections::HashMap, sync::{mpsc, RwLock}, thread};

#[derive(Debug)]
pub struct Bank {
    pub id: u32,
    pub balance: u32,
    pub accounts: RwLock<HashMap<u32, BankAccount>>,
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
                    Bank::handle_transation(id, transaction);
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
            accounts: RwLock::new(accounts),
            thread,
            sender: tx,
        }
    }

    /// Handle a transaction given in params
    /// Change balance of accounts in one bank or in differnt banks
    /// Bank change account balance if only it is the target bank in transaction
    fn handle_transation(bank_id: u32, transaction: Transaction) {
        // get HashMap of all banks
        println!("start handle transaction"); // TODO: fix WouldBlock
        // get bank and account where value will be substructed
        let mut bank_from = BANKS.get(&bank_id).unwrap().accounts.write().unwrap();
        let account_from = bank_from.get_mut(&transaction.account_id).unwrap();

        // if it is internal transaction, add value to another account in this bank
        if transaction.target_bank_id == transaction.bank_from {
            // substruct value
            account_from.substruct_from_balance(transaction.value);

            // get target account
            let account_to = bank_from
                .get_mut(&transaction.target_account_id)
                .unwrap();
            account_to.add_to_balance(transaction.value);
        }
        else if bank_id == transaction.bank_from {
            account_from.substruct_from_balance(transaction.value);
            let mut bank_to = BANKS.get(&transaction.target_bank_id).unwrap();
            bank_to.send_transaction(transaction);
        }
        // else send this transaction to another bank
        else if bank_id == transaction.target_bank_id {
            let account_to = bank_from
                .get_mut(&transaction.target_account_id)
                .unwrap();
            account_to.add_to_balance(transaction.value);
        }
        

    }
    
    /// Sends transaction to bank given in params
    pub fn send_transaction(&self, transaction: Transaction) {
        self.sender.send(transaction).unwrap();
    }
}
