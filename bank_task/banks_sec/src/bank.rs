use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

//if our client want to transfer his account to another ...
//... first he chosee his account that he want to transder ...
//... then we look to bank_id in struct BankAccount an d find the bank where this Acc is stored...
//... and then reilize the Transfer to another Acc

//declare the struct if bank
#[derive(Debug)]
pub struct Bank{
    id: usize,
    balance:usize,//balance of bank
    client_accounts: Vec<BankAccount>,//all accounts in the bank
    thread: Option<thread::JoinHandle<()>>,
}

impl Bank{
    //create method to create new instance of Bank value - count of Accounts in bank
    fn new(id_in:usize,mut value:usize,receiver: Arc<Mutex<mpsc::Receiver<Target>>>) -> Bank{
        let mut accounts:Vec<BankAccount> = vec![];
        let mut bal:usize = 0;
        for o in 0..value{
            accounts.push(BankAccount { accound_id: o, bank_id: id_in, value: 100 });
            bal+=100;
        }
        

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(transfer) => {
                    for i in 0..value{
                        if transfer.target_id == i{
                            accounts[i].value-= transfer.value;
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        });
        Bank{
            id:id_in,
            balance:0,
            client_accounts:accounts,
            thread:Some(thread),
        }
    }
}

//declare the struct if client
pub struct Client{
    client_id:usize,
    accounts: Vec<BankAccount>,//all client accounts in banks
    // thread: Option<thread::JoinHandle<()>>,
}

impl Client{
    //create method to create new instance of Client
    pub fn new(id_in:usize) -> Client{
        let accounts:Vec<BankAccount>;
        Client{
            client_id:id_in,
            accounts:vec![],
            // thread:Some(thread),
        }
    }

    pub fn create_transfer(target_out:usize,target_in:usize,bank_in:usize,amount_to_transfer:usize) -> TransferBetweenAccounts{
        TransferBetweenAccounts { target_out, bank_in, target_in, amount_to_transfer}
    }

    pub fn pool_req(pool:ThreadPool){

    }
}


//declare the struct if bank account
#[derive(Debug)]
pub struct BankAccount{
    accound_id:usize,//id of account in the bank
    bank_id:usize,//id of the bank where the account is stored
    value:usize,//value of account
}

impl BankAccount{
    //create method to create new instance of BankAccount
    fn new(id_in:usize,balance:usize,bank_id_in:usize) -> BankAccount{
        BankAccount{
            accound_id:id_in,
            value:balance,
            bank_id:bank_id_in,
        }
    }
}

pub struct Target{
    target_id:usize,
    value:usize,
}


//declare struct of transfer
pub struct  TransferBetweenAccounts{
    pub target_out:usize,//the Account from which the value will be transferred
    pub bank_in:usize,
    pub target_in:usize,//the Account to which the value will be transferred
    pub amount_to_transfer:usize,// value to transfer
}



impl TransferBetweenAccounts{
    //create method to create new instance of Transfer
    fn new(acc_out:usize,acc_in:usize,value:usize,bank_in:usize) -> TransferBetweenAccounts{
        TransferBetweenAccounts { 
            target_out: acc_out,
            bank_in,
            target_in:acc_in,
            amount_to_transfer: value, 
        }
    }
}

















//lazy static global_banks


pub struct ThreadPool {
    banks:Vec<Bank>,
    sender: Vec<Option<mpsc::Sender<Target>>>,
}

impl ThreadPool {
    pub fn printAll(&self){
        for i in 0..self.banks.len(){
            println!("{:?}",self.banks[i])
        }
    }

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut banks = Vec::with_capacity(size);
        let mut sender = Vec::with_capacity(size);


        for id in 0..size {
            let (sender_b, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            sender.push(Some(sender_b));
            banks.push(Bank::new(id, id*100 ,(receiver)));
        }


        //global_banks = banks
     



        ThreadPool {
            banks,
            sender,
        }
    }

    pub fn execute(&self, request:TransferBetweenAccounts,bank_id:usize)
    {
        let mut target_bank :&Bank ;
        let mut req :Target = Target { target_id: request.target_out, value: request.amount_to_transfer };
        let mut req_back :Target = Target { target_id: request.target_in, value: 1 - request.amount_to_transfer };
        for i in 0..self.banks.len(){
            let mut bank = &self.banks[i];
            if bank.id == bank_id{
                target_bank = bank;
                self.sender[i].as_ref().unwrap().send(req).unwrap();
                break;
            }
        }

        let mut target_bank_sec :&Bank ;
        
        for i in 0..self.banks.len(){
            let mut bank = &self.banks[i];
            if bank.id == request.bank_in{
                target_bank = bank;
                self.sender[i].as_ref().unwrap().send(req_back).unwrap();
                break;
            }
        }

    }
}

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         drop(self.sender.take());
//         for bank in &mut self.banks {
//             println!("Shutting down bank {}", bank.id);

//             if let Some(thread) = bank.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }