//Standard buffer
use std::io::{BufRead, BufReader};

//Standard file operations
use std::fs::File;
use std::string::String;

//Standard random functions
use rand;
use rand::Rng;

//Message Of The Day
pub fn motd() -> String {
    //File operations
    let file = File::open("motd.txt").unwrap();
    let buffer = BufReader::new(&file);
    let mut phrases: Vec<String> = Vec::new();

    for line in buffer.lines() {
        phrases.push(line.unwrap().to_string());
    }

    //Getting random index
    let random_index = rand::thread_rng().gen_range(0, phrases.len() - 1);

    return phrases[random_index].clone();
}