mod parse;
mod rat;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("No arguments provided!");

    // Initialize the file
    let file = parse::parse_file(path);
    let start = parse::find_start(&file);

    // Initialize rat
    let r = rat::new(start[0], start[1]);

    // Start the maze!
    rat::run_maze(r, file);
}