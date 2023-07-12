use std::error::Error;
use std::fs;
use std::env;

//function that read file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {//if count without upper/lower case 
        search_case_insensitive(&config.query, &contents)
    } else {//if count with upper/lower case
        search(&config.query, &contents)
    };
    //print our results
    for line in results {
        println!("{line}");
    }
    Ok(())
}



//declare Config struct
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    //create new entity of config with arguments from command line
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");//throw error
        }

        let query = args[1].clone();//2 elem in args 
        let file_path = args[2].clone();//3 elem in args(because the 1 elem is the path to the project)
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path ,ignore_case})
    }
}

// //function that parce arguments and return entity of Config struct
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();//2 elem in args 
//     let file_path = args[2].clone();//3 elem in args(because the 1 elem is the path to the project)

//     Config { query, file_path }
// }

//function return strings that contains a key word
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
//function do same thing like function above, but dont xhek lower/upper case
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

//: Creating a failing test for the search function we wish we had


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}