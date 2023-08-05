mod parse;

use std::env;
use std::fs::File;
use std::io::{BufRead, Result, BufReader, Lines};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("No arguments!");

    let file = parse::parse_file(path);

    println!("{}", file.join("\n"));
}