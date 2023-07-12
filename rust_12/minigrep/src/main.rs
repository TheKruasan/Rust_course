use std::env;
#[allow(unused)]
fn main_s() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}



fn main() {
    let args: Vec<String> = env::args().collect();//read args from console

    let query = &args[1];//2 elem in args 
    let file_path = &args[2];//3 elem in args(because the 1 elem is the path to the project)

    println!("Searching for {}", query);
    println!("In file {}", file_path);

}