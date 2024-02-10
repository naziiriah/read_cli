use std::fs;
use std::error::Error;
use std::env;

pub fn run(config:Config) ->Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for line in results{
        println!("{}",line);
    }

    Ok(())

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config{
   pub fn new(args:&[String]) ->Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive= env::var("CASE_INSENSITIVE").is_err();

        
        Ok(Config{query, filename, case_sensitive})
    }
}


pub fn search<'a>(query:&str, content:&'a str)-> Vec<&'a str> {
    let mut result  = Vec::new();
    for lines in content.lines() {
         if lines.contains(&query){
            result.push(lines);
         }
    }
    result
}

pub fn search_case_insensitive<'a>(query:&str, contents:&'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
  
    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()){
           result.push(lines);
        }
   }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test(){
        let query= "banish us";
        let content = "
        I'm a nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
They'd banish us, you know.
        How dreary to be somebody!
        How public, like a frog
        To tell your name the livelong dreary
        To an admiring bog! ";
    assert_eq!(vec!["They'd banish us, you know."], search(query, content))
    }  

    #[test]
    fn case_insensitive (){
        let query = "ruSt";
        let contents = "\
        Rust:
        safe, fast, productive,
        pick three,
Trust me.";

        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
