use std::env;
use std::process;
// connetcting to lib
use updated_project::Config;

fn main() {

    // set configuration
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // check if there is error in running program
    if let Err(e) = updated_project::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}