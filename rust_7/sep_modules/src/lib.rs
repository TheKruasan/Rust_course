mod front_of_house;//module 

pub use crate::front_of_house::hosting;//link to module hosting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();//using function described in module hosting
}