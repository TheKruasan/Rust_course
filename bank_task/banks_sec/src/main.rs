#[macro_use]
extern crate lazy_static;

pub mod bank;
use std::f32::consts::E;

use bank::*;


fn main() {
    let mut clients:Vec<Client> = vec![];
    for id in 0..10 {
        clients.push(Client::new(id));
    }
    let pool = ThreadPool::new(4);

    loop {
        let mut transfer:TransferBetweenAccounts=TransferBetweenAccounts{target_out: 3,target_in: 1,bank_in: 2,amount_to_transfer: 40};
        pool.execute(transfer,2);
    }
}
