mod parse;
mod rat;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("No arguments!");

    let file = parse::parse_file(path);
    let start = parse::find_start(&file);

    // DEBUG INFO: TO BE DELETED LATER
    println!("{}", file.join("\n"));
    println!("Start Position: {}, {}", start[0], start[1]);

    let r = rat::new(start[0], start[1]);

    rat::run_maze(r, file);

    println!("Maze Completed!!!!");
}