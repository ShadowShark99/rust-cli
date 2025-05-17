use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.file_path)
  .expect("shoudl have been able to read the file");

  //println!("with text :\n{contents}");

  Ok(())
}

pub struct Config{
  pub query: String,
  pub file_path: String,
}

impl Config{
  pub fn build( args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("Usage: minigrep <query> <file_path>");
          //std::process::exit(1);
      }
  
      Ok(Config{
          query: args[1].clone(),
          file_path: args[2].clone(),
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
}