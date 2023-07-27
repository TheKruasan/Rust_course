use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
#[derive(Debug)]
pub struct Bank{
}

impl Bank{
    fn new() -> Bank{
        Bank{
        }
    }
}
pub struct Client{
}

impl Client{
    pub fn new() -> Client{
        Client{
        }
    }
}

#[derive(Debug)]
pub struct BankAccount{
}

impl BankAccount{
    fn new(id_in:usize,balance:usize,bank_id_in:usize) -> BankAccount{
        BankAccount {  }
    }
}


pub struct Target{//for transfer between banks
}


//declare struct of transfer
pub struct  TransferBetweenAccounts{
}
impl TransferBetweenAccounts{
    fn new() -> TransferBetweenAccounts{
        TransferBetweenAccounts {  }
    }
}