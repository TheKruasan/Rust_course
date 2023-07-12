fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    // add key anf value 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    //get value by key
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);



    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn hash_kards(){
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}


fn change_value(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    //change value by key
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);



    println!("{:?}", scores);

    //add value with method entry it works if we have not this key in our hash map   
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

}


fn create_new_value_based_on_old_value(){
    use std::collections::HashMap;

    let text = "hello world wonderful world";//old value

    let mut map = HashMap::new();

    for word in text.split_whitespace() {//split our old value and iterate on the result
        let count = map.entry(word).or_insert(0);// add to new hash_map new values
        *count += 1;
    }

    println!("{:?}", map);
}