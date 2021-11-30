# Advent Of Code

A simple framework to include in all Advent of Code solutions in a single
framework to make it easy to call it with the appropriate CLI arguments.

## Installation

Have a proper full Rust installation (see [rustup.rs](rustup.rs) for more).
Clone this repository such as via:

```zsh
git clone https://github.com/OvermindDL1/advent_of_code.git
cd advent_of_code
```

Then build it by:

```zsh
cargo build --release
```

## Usage

If following the above instructions then the final program will be at:

```zsh
./target/release/advent_of_code
```

And as such it will be used as the executable path, feel free to move
it elsewhere.

See the help with the `--help` flag or the `help` command such as:

```zsh
./target/release/advent_of_code --help
```

The `--help` flag and `help` command works on other commands as well,
such as seeing the help for one of the years by:

```zsh
./target/release/advent_of_code 2020 --help
```

To run, for example, the Advent of Code 2020 Day 1 then run:

```zsh
./target/release/advent_of_code 2020 1 ./inputs/2020/day1.input
```

Or use your own input to get your own answer (please try to answer
Advent Of Code on your own, don't just copy and paste answers from
elsewhere).

You can see what arguments are needed for a task by showing its
`--help` like such:


```zsh
./target/release/advent_of_code 2020 1 --help
```

To see the running time of the command then add the `-v` for verbose
as the first argument, such as with:

```zsh
./target/release/advent_of_code -v 2020 1 ./inputs/2020/day1.input
```

Feel free to add more `v`'s such as `-vvv` for more detailed logging,
but just one will print the times for the runs.