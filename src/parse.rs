use std::io::{Result, Lines, BufReader, BufRead};
use std::path::Path;
use std::fs::File;

pub fn parse_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>, {

    let lines = read_lines(filename).unwrap();
    let mut parsed: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(s) = line {
            parsed.push(s)
        }
    }

    parsed
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}