// definition of struct named User
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//definition of tuple struct without named the atributes
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// example of unit-like struct. Use this when you 
// dont have any data that you want to store in the type itself
struct AlwaysEqual;

// this function create user instance
// from email and name given in params
//it works if we have parameter and atribute with same name
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //shortheand instead of "username: username",
        email,    //shortheand instead of "email: email",
        sign_in_count: 1,
    }
}

fn main() {
    // creating an instance of User by specifying concrete
    // values for each of the field
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // mmutable instance of User
    let mut user2: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // access to field using dot notation
    user2.email = String::from("anotheremail@example.com");

    // example of creating user instance with own function
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // creating instance from other instance
    // using dot notation to access fields
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // creating instance from other instance
    // using `..` syntax
    let user3 = User {
        // set explicitly
        email: String::from("another@example.com"),
        // remaining fields will be the as same as in user1
        ..user2//same atributes like in user2 
    };

    // creating instances of tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // creating instances of unit-like struct
    let subject = AlwaysEqual;
}