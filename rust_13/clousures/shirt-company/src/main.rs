// describes all shirt colors that company have
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

// describes all shirts copmpany have
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // return a shirt
    // if user preference is not None, it will give prefered color
    // else it will give the most stocked color
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // using clousure without args here
    }

    // returns the most stocked color
    fn most_stocked(&self) -> ShirtColor {
        // number of shirts 
        let mut num_red = 0;
        let mut num_blue = 0;
        // counting amount of shirts with their colors
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        // returns shirt with the most stocked color
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // init the store with shirts
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // init user preference
    let user_pref1 = Some(ShirtColor::Red);
    // first giveaway
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    // second giveaway
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}