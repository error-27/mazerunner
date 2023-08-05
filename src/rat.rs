// Functions and datatypes for the rat

struct Rat {
    x: u8,
    y: u8,
    dir: Direction,
    hunger: u8,
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

            _ => eprintln!("how the hell did you make a fifth cardinal direction") // Literally impossible to get this
        }
    }
}