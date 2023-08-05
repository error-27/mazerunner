# mazerunner
An esoteric programming language where you build a maze for a rat.

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
- Any unreachable areas will not be read and can be considered comments.
- Each character in the maze executes an instruction, except some like `+`.

## Instruction Set (INCOMPLETE)
| Character | Description                                       |
|-----------|---------------------------------------------------|
| `+`       | Acts as a basic path character that does nothing. |
| `S`       | Starting point for the rat.                       |
| `c`       | Cheese that must be placed every 10 steps.        |
| `C`       | Big cheese that ends the program.                 |
| TODO      | TODO                                              |