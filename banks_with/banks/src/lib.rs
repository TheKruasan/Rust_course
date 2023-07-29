use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref BANKS: RwLock<HashMap<u32, Bank>> = {
        let mut m = HashMap::new();
        RwLock::new(m)
    };
}
#[derive(Debug)]
pub struct Bank {
    pub id: u32,
    pub balance: u32,
    pub accounts: HashMap<u32, BankAccount>,
    // pub thread: Option<thread::JoinHandle<()>>,
}

impl Bank {
    fn new(id: u32, balance: u32, accounts: HashMap<u32, BankAccount>) -> Bank {
        // let thread = thread::spawn(move || loop {
        //     let message = receiver.lock().unwrap().recv();

        //     match message {
        //         Ok(job) => {
        //             println!("Worker {id} got a job; executing.");

        //             job();
        //         }
        //         Err(_) => {
        //             println!("Worker {id} disconnected; shutting down.");
        //             break;
        //         }
        //     }
        // });

        Bank {
            id,
            balance,
            accounts,
            // thread: Some(thread),
        }
    }

    pub fn add_request(&mut self, request: TransferBetweenAccounts) {
        let target: Target = Target {
            target_account: request.target_account_id,
            value: request.value,
        };

        if request.target_bank_id != self.id {
            self.accounts
                .get_mut(&request.account_id)
                .expect("error in add_request")
                .substruct_from_balance(request.value);
            self.send_target_to_another_bank(request.target_bank_id, target)
        } else {
            self.accounts
                .get_mut(&request.account_id)
                .expect("error in add_request")
                .substruct_from_balance(request.value);
            self.accounts
                .get_mut(&request.target_account_id)
                .expect("error in add_request")
                .add_to_balance(request.value);
        }
    }

    pub fn send_target_to_another_bank(&self, target_bank_id: u32, target: Target) {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // TODO: fix it
        BANKS
            .read()
            .expect("error in send_target_to_another_bank")
            .get(&target_bank_id)
            .expect("error in send_target_to_another_bank")
            .set_reciever(Arc::clone(&receiver));
        println!("in send_target_to_another_bank");
        sender
            .send(target)
            .expect("error in send_target_to_another_bank");
    }

    pub fn set_reciever(&self, receiver: Arc<Mutex<mpsc::Receiver<Target>>>) {
        let mut accounts = self.accounts.clone();

        thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            println!("got message");
            match message {
                Ok(job) => {
                    accounts
                        .get_mut(&job.target_account)
                        .expect("error in set_reciever")
                        .change_balance(0 - job.value);
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        });
    }
}
pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }
}

#[derive(Clone, Debug)]
pub struct BankAccount {
    pub id: u32,
    pub balance: u32,
}

impl BankAccount {
    pub fn change_balance(&mut self, value: u32) {
        self.balance -= value;
    }

    pub fn substruct_from_balance(&mut self, value: u32) {
        self.balance -= value;
    }

    pub fn add_to_balance(&mut self, value: u32) {
        self.balance += value;
    }
}

pub struct Target {
    target_account: u32,
    value: u32, //for transfer between banks
}

//declare struct of transfer
pub struct TransferBetweenAccounts {
    pub account_id: u32,
    pub target_bank_id: u32,
    pub target_account_id: u32,
    pub value: u32,
}
// impl TransferBetweenAccounts{
//     fn new() -> TransferBetweenAccounts{
//         TransferBetweenAccounts {  }
//     }
// }