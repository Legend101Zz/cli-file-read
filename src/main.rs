use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");


    println!("With text: \n{}", contents)
}


struct Config {
query : String,
filename : String,

}


fn parse_config(args: &[String]) -> Config {


    let query = &args[1];
    let filename = &args[2];
 
Config{query , filename}
}