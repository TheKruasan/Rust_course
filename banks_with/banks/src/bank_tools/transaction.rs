#[derive(Debug)]
pub struct Transaction {
    pub bank_from:u32,
    pub account_id: u32,
    pub target_bank_id: u32,
    pub target_account_id: u32,
    pub value: u32,
}