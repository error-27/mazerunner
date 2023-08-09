// Functions and datatypes for the rat

use crate::parse::is_valid;

pub struct Rat {
    x: u32,
    y: u32,
    dir: Direction,
    hunger: u8,
    food_cooldown: u8,
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn new(x: u32, y: u32) -> Rat {
    Rat {
        x,
        y,
        dir: Direction::Up,
        hunger: 0,
        food_cooldown: 0,
    }
}

pub fn run_maze(mut rat: Rat, lines: Vec<String>) {
    // Initialize stack and accumulators.
    let mut stack: Vec<u8> = Vec::new();
    let mut accum_a: u8 = 0;
    let mut accum_b: u8 = 0;

    loop {
        let current: char = lines[rat.y as usize].chars().nth(rat.x as usize).unwrap();

        // Run current instruction
        match current {
            'C' => {
                println!("Maze Completed!"); // Big Cheese has been reached. End the program.
                break;
            }
            'c' => {
                rat.hunger = 0;
                rat.food_cooldown += 1;
            }
            'a' => {
                accum_a += 1;
            }
            'b' => {
                accum_b += 1;
            }
            'A' => {
                accum_a -= 1;
            }
            'B' => {
                accum_b -= 1;
            }

            _ => {} // Catch anything unknown or that doesn't run code.
        }

        if rat.hunger >= 10 {
            println!("Rat starved at position ({}, {})!", rat.x, rat.y);
            break;
        }
        if rat.food_cooldown >= 3 {
            println!("Rat overate at position ({}, {})", rat.x, rat.y);
            break;
        }

        rat.hunger += 1;
        if rat.hunger % 5 == 0 && rat.food_cooldown > 0 {
            rat.food_cooldown -= 1;
        }

        // Calculate north, south, east, and west chars.
        let n: Option<char> = if rat.y != 0 {
            lines[(rat.y - 1) as usize].chars().nth(rat.x as usize)
        } else {
            None
        };
        let w: Option<char> = if rat.x != 0 {
            lines[rat.y as usize].chars().nth((rat.x - 1) as usize)
        } else {
            None
        };
        let e: Option<char> = if lines[rat.y as usize].len() != (rat.x + 1) as usize {
            lines[rat.y as usize].chars().nth((rat.x + 1) as usize)
        } else {
            None
        };
        let s: Option<char> = if lines.len() != (rat.y + 1) as usize {
            lines[(rat.y + 1) as usize].chars().nth(rat.x as usize)
        } else {
            None
        };

        // Rat movement code
        match rat.dir {
            Direction::Up => {
                if is_valid(&w) {
                    rat.dir = rat.dir.left();
                    rat.x -= 1;
                } else if is_valid(&n) {
                    rat.y -= 1;
                } else if is_valid(&e) {
                    rat.dir = rat.dir.right();
                    rat.x += 1;
                } else if is_valid(&s) {
                    rat.dir = rat.dir.right().right();
                    rat.y += 1;
                } else {
                    println!("Rat is stuck!");
                    break;
                }
            }
            Direction::Left => {
                if is_valid(&s) {
                    rat.dir = rat.dir.left();
                    rat.y += 1;
                } else if is_valid(&w) {
                    rat.x -= 1;
                } else if is_valid(&n) {
                    rat.dir = rat.dir.right();
                    rat.y -= 1;
                } else if is_valid(&e) {
                    rat.dir = rat.dir.right().right();
                    rat.x += 1;
                } else {
                    println!("Rat is stuck!");
                    break;
                }
            }
            Direction::Down => {
                if is_valid(&e) {
                    rat.dir = rat.dir.left();
                    rat.x += 1;
                } else if is_valid(&s) {
                    rat.y += 1;
                } else if is_valid(&w) {
                    rat.dir = rat.dir.right();
                    rat.x -= 1;
                } else if is_valid(&n) {
                    rat.dir = rat.dir.right().right();
                    rat.y -= 1;
                } else {
                    println!("Rat is stuck!");
                    break;
                }
            }
            Direction::Right => {
                if is_valid(&n) {
                    rat.dir = rat.dir.left();
                    rat.y -= 1;
                } else if is_valid(&e) {
                    rat.x += 1;
                } else if is_valid(&s) {
                    rat.dir = rat.dir.right();
                    rat.y += 1;
                } else if is_valid(&w) {
                    rat.dir = rat.dir.right().right();
                    rat.x -= 1;
                } else {
                    println!("Rat is stuck!");
                    break;
                }
            }
        }
    }
}