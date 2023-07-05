fn main() {

    //we can create a function that print smth if param type is Some
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }


    //using if let control flow with to rewrite function above
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }



    //function with match that print smth if coin have a Quater type
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),//Quater is a type of data in enum Coin ...
                                                                            //... in control_flow_math folder
        _ => count += 1,
    }



    //using if let control flow to rewrite function above
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
