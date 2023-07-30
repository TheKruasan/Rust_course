use crate::bank_tools::bank::Bank;
use lazy_static;
use std::{collections::HashMap, sync::{RwLock, Arc}};

lazy_static! {
    // Define a [`HashMap`] with all banks 
    pub static ref BANKS: Arc<RwLock<HashMap<u32, Bank>>> = {
        let map = HashMap::new();
        Arc::new(RwLock::new(map))
    };
}
