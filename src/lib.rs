use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.file_path)
  .expect("shoudl have been able to read the file");

  let results = if config.ignore_case{
    search_case_insensitive(&config.query, &contents)
  }else{
    search(&config.query, &contents)
  };

  for line in results{
    println!("{line}");
  }

  Ok(())
}

pub struct Config{
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config{
  pub fn build( args: &[String]) -> Result<Config, &'static str> {
      let mut ignore_case = env::var("IGNORE_CASE").is_ok();
      if args.len() < 3 || args.len() > 4 {
          return Err("Usage: minigrep <query> <file_path> [--ignore-case]");
          //std::process::exit(1);
      }
      else if args.len() == 4{
        // command line arg has higher precedence than env var
          ignore_case = args[3] == "--ignore-case" || args[3] == "1";
      }
  
      Ok(Config{
          query: args[1].clone(),
          file_path: args[2].clone(),
          ignore_case,
      })
  }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
  let mut matches = Vec::new();
  for line in contents.lines(){
      if line.contains(query){
          matches.push(line);
      }
  }

  matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
  let mut matches = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines(){
      if line.to_lowercase().contains(&query){
          matches.push(line);
      }
  }
  matches
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn one_result() {
      let query = "duct";
      let contents = "\
Rust:
safe, fast, productive.
Pick three.";
      assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive(){
    let query = "balls";
    let contents = "\
    BALLS
    are Balling.";
    assert_eq!(vec!["BALLS"], search_case_insensitive(query, contents));
  }
}