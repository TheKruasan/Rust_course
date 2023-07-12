fn main() {
    let mut s = String::new();


    let data = "initial contents";
    //use method to_string to start the string with some data
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
    let s = String::from("initial contents");

    //this strings are available in rust 
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    //add smth to the string with push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    //we change s1 but dont change s2 
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //method push add just one symbol to string
    let mut s = String::from("lo");
    s.push('l');




    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used




    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //the complicated string combining use format! macro 
    let s = format!("{s1}-{s2}-{s3}");
    //to same things like this:
    let s = s1 + "-" + &s2 + "-" + &s3;



    let s1 = String::from("hello");
    let h = s1[0];//error code


    //slice of the string
    let hello = "Здравствуйте";
    //2 bytes
    let s = &hello[0..4];
    //s = "Зд"

    //iteration on string using chars
    for c in "Зд".chars() {
        println!("{}", c);
    }

    //iteration on string using bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }

}
