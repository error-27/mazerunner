use std::io::{Result, Lines, BufReader, BufRead};
use std::path::Path;
use std::fs::File;

// Parse file into a Vec for easy access of different parts.
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

// Return the first found S character's position.
pub fn find_start(lines: &Vec<String>) -> [u32; 2] {
    let mut position = [0u32, 0u32];

    let mut iter = 0;
    for line in lines {
        if let Some(p) = line.find('S') {
            position[0] = p as u32;
            position[1] = iter;
        }
        iter += 1;
    }

    position
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn is_valid(c: &Option<char>) -> bool {
    c.is_some() && c.unwrap() != ' '
}