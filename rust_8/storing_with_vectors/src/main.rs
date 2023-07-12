fn main() {
    //create new empty vector to store values with type i32
    let v: Vec<i32> = Vec::new();
    //create not empty vector without named type
    let v = vec![1, 2, 3];
    //create mutable vector and then change it  
    let mut v = Vec::new();
    //add new elements in vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    //Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //iterate over the vector and change it
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    //delete elems in vector
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}



fn read_data_from_vector(){
    //create new vector
    let v = vec![1, 2, 3, 4, 5];

    //get value from vector with indexing syntax 
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    //get value from vector with method get
    let third: Option<&i32> = v.get(2);
    
    //handle information that we get from method get
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}



fn using_an_enum_to_store_multiple_types(){
    enum SpreadsheetCell {//declaring an enum
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![//create vector that store multiple types
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}