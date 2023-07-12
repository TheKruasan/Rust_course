use std::env;
use std::process;

use read_from_file::Config;

fn main() {   

    let args: Vec<String> = env::args().collect();//read args from console
    //cahge it let (query, file_path) = parse_config(&args) to it:
    //let config = parse_config(&args) then change it to it:
    //let config = Config::new(&args) then to it:
   
    let config = Config::build(&args).unwrap_or_else(|err| { //parce arguments
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = read_from_file::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}







