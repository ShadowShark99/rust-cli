use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: \n{err}");
        //println!("Usage: minigrep <query> <file_path>");
        std::process::exit(1);
    });
    //println!("fname: {}", fname);
    //println!("Searching for grep: {}", config.query);
    //println!("file_path: {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        println!("Application error: {e}");
        std::process::exit(1);
    }


}



