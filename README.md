# Conways-Game-Of-Life

## Description

Implementation in Rust of Conway's Game of Life for the Hacking Learning Path in LambdaClass

### Conways Game Of Life

The universe of the Game of Life is an infinite two-dimensional orthogonal grid of square cells, each of which is in one of two possible states, alive or dead, or "populated" or "unpopulated". Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. At each step in time, the following transitions occur:

- Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.

- Any live cell with two or three live neighbours lives on to the next generation.

- Any live cell with more than three live neighbours dies, as if by overpopulation.

- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

The initial pattern constitutes the seed of the system. The first generation is created by applying the above rules simultaneously to every cell in the seed—births and deaths occur simultaneously, and the discrete moment at which this happens is sometimes called a tick (in other words, each generation is a pure function of the preceding one). The rules continue to be applied repeatedly to create further generations.

## Usage

You should have Rust and Cargo installed

Clone this repository and execute the following command to run

`make run csv_path=<path_to_csv_file> milliseconds=<time_between_frames_in_milliseconds>`

Use the following command to run tests

`make test`

### CSV Format

The CSV file taken as input should have the following format, representig the initial state of the grid

```
1,0,0,0,
0,0,0,1,
0,0,0,0,
1,0,0,0,
```

With 1 representing an alive cell, and 0 a dead cell.

## Examples

Three examples are provided under the examples folder, you can run them as follows

- Glider: `make glider`
- Space Ship: `make space_ship`
- Glider Gun: `make glider_gun`

## Error Handling

The following errors can occur, they show a message on screen detailing the error

- File not Found: `make run csv_path=examples/not_existing_file.csv milliseconds=500`
- Invalid number for time argument: `make run csv_path=examples/glider.csv milliseconds=a`
- Incorrect number of arguments: `cargo run csv_path=examples/glider.csv`
- Incorrect formatting of CSV:
    - `make run csv_path=examples/wrong_format.csv milliseconds=100`
    - `make run csv_path=examples/wrong_format_2.csv milliseconds=100`
    - `make run csv_path=examples/wrong_format_3.csv milliseconds=100`

## References

[Conway's Game of Life's Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) 

[Youtube Video with Examples](https://www.youtube.com/watch?v=C2vgICfQawE) 

