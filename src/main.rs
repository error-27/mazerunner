mod parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("No arguments!");

    let file = parse::parse_file(path);
    let start = parse::find_start(&file);

    println!("{}", file.join("\n"));
    println!("{}, {}", start[0], start[1]);
}