use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();

    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
