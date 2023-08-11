# mazerunner
An esoteric programming language where you build a maze for a rat.

## TODO
* [x] Instructions to pop from stack and copy to an accumulator.
* [ ] Instruction to take text input and put it in the stack.
* [x] Control flow instruction that allows left if both accumulators are equal.

## The Rat
- The rat starts at the `S` character and tries to reach the `C` character.
- The rat must eat a cheese (`c` character) every ten steps or the program stops.
- Overfeeding the rat kills it and stops the program.
- The rat will always try to turn left at any intersections. If it can't turn left, it will turn forward, then right, and finally if it can't turn, it will go backwards.
- The rat runs instructions as it reaches them.
- Upon reaching the `C` character, the program ends.
- The rat starts facing upwards.

## The Maze
- The maze is constructed out of valid command characters.
- It is always closed in and must contain an `S`.
- Any unreachable areas will not be read and can be considered comments. It is recommended to place comments above the maze though so the start position is never selected from a comment.
- Each character in the maze executes an instruction, except some like `+`.

## Data
- Push and pop unsigned 8-bit integers to a stack.
- Two unsigned 8-bit accumulators can be written to and read from.

## Instruction Set (INCOMPLETE)
| Character | Description                                                       |
|-----------|-------------------------------------------------------------------|
| `+`       | Acts as a basic path character that does nothing.                 |
| `S`       | Starting point for the rat.                                       |
| `c`       | Cheese that must be placed every 10 steps.                        |
| `C`       | Big cheese that ends the program.                                 |
| `a`       | Adds 1 to accumulator A.                                          |
| `b`       | Adds 1 to accumulator B.                                          |
| `A`       | Subtracts 1 from accumulator A.                                   |
| `B`       | Subtracts 1 from accumulator B.                                   |
| `z`       | Reset accumulator A to 0.                                         |
| `Z`       | Reset accumulator B to 0.                                         |
| `T`       | Stops rat from moving left if accumulator B is not 0.             |
| `Y`       | Stops rat from moving left if accumulators A and B are not equal. |
| `P`       | Pushes accumulator A to the stack.                                |
| `p`       | Pops from the stack and outputs as a number.                      |
| `r`       | Pops from the stack and outputs as a character.                   |
| `R`       | Pops the whole stack and outputs as a string.                     |
| `o`       | Pops from the stack and copies to accumulator A.                  |
| `O`       | Pops from the stack and copies to accumulator B.                  |
| `d`       | Pops from the stack and discards the value.                       |
| `^`       | Northward one-way ramp. The rat can't enter this from the north.  |
| `>`       | Eastward one-way ramp. The rat can't enter this from the east.    |
| `<`       | Westward one-way ramp. The rat can't enter this from the west.    |
| `v`       | Southward one-way ramp. The rat can't enter this from the south.  |
