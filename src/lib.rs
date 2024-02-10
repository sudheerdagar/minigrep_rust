use std::error::Error;
use std::fs; //module used for file reading
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;


    let results=if config.case_senstive{
        search(&config.query,&contents)
    } else{
        search_case_insenstive(&config.query,&contents)
    };


    for line in results{
        println!("{}",line);
        
    }





    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_senstive:bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_senstive=env::var("CASE_INSENSTIVE").is_err();


        Ok(Config{query,filename,case_senstive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

   


    results
}

pub fn search_case_insenstive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query=query.to_lowercase();


    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] //this attribute tell us that it is a test function
    fn one_result() {
        let query = "duct";
        let contents: &str = "\
    Rust:
safe,fast,productive.
    pick three.";

        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }

    #[test]
    fn case_insenstive(){

        let query: &str="rUst";

        let contents:&str= "\
    Rust:
    safe,fast,productive.
    Pick three.
Trust me.";

    assert_eq!(vec!["Rust:","Trust me."], search_case_insenstive(query, contents));

    }
}
