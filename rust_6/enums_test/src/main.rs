//Express this concept in code by defining an ...
//... IpAddrKind enumeration and listing the possible kinds ...
//... an IP address can be, V4 and V6. These are the variants of the enum:
fn main(){
    //defining enum that describe all defferent values of types 
    enum IpAddrKind {
        V4,
        V6,
    }
    enum IpAddr {
        V4(String),
        V6(String),
    }
    //create two instances of the struct IpAddr
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    
    
    //create instances of each of the two variants of IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //Call this function with either variant:
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    //empty function just for test the 23 and 24 line
    fn route(ip_kind: IpAddrKind) {}

    //each variant can have different types and amounts of associated data
    enum IpAddrTh {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    //create instances of enum IpAddrTh
    let home = IpAddrTh::V4(127, 0, 0, 1);
    let loopback = IpAddrTh::V6(String::from("::1"));
    //This code illustrates that you can put any kind of data inside an enum variant: ... 
    //... strings, numeric types, or structs, for example. You can even include another enum
    
    struct Ipv4Addr {// some random struct
        // --snip--
    }

    struct Ipv6Addr {// second random struct
        // --snip--
    }

    enum IpAddrS {
        // use random struct like type of data
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // define emun that use different types structs like a king of data
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }   
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    main_2()
}



fn main_2() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    //create a method call to enum with block impl
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    //create a instance of message enum
    let m = Message::Write(String::from("hello"));
    //use method call that we describe in message enum
    m.call();

    //define a Option emun
    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //we can't use this code
    // because the type i8 isnt consist in Option enum
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    //end of error code
}