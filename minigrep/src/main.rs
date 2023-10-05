use std::env;
use std::fs;

fn main() {
    // create a vector of strings
    // the args function returns an iterator over the arguments passed to the program
    // the collect function will turn that iterator into a collect function
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query} in file {filename}");

    // if file found, will return contents of file as a string
    // if file not found, will return an error
    let contents = fs::read_to_string(filename)
        .expect("The file was not read successfully");

    println!("{contents}")
}