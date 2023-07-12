use std::env;
use std::process;

use minigrep::Config;

fn main() {   

    let args: Vec<String> = env::args().collect();//read args from console
    //cahge it let (query, file_path) = parse_config(&args) to it:
    //let config = parse_config(&args) then change it to it:
    //let config = Config::new(&args) then to it:
   
    let config = Config::build(&args).unwrap_or_else(|err| { //parce arguments
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

//function that read file
fn run(config: Config) {
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())


}


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");//throw error
        }

        let query = args[1].clone();//2 elem in args 
        let file_path = args[2].clone();//3 elem in args(because the 1 elem is the path to the project)

        Ok(Config { query, file_path })
    }
}

//function that parce arguments and return entity of Config struct
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();//2 elem in args 
    let file_path = args[2].clone();//3 elem in args(because the 1 elem is the path to the project)

    Config { query, file_path }
}




