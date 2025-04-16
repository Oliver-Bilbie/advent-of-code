# 🎄 Advent of Code 🎄

Solutions to [Advent of Code](https://adventofcode.com/) programming challenges for various years.

## How to run the programs

### Web app

The simplest way to use the programs is to visit the web app [hosted here](https://aoc.oliver-bilbie.co.uk/) which provides an interface for running the solutions directly from a web browser using WebAssembly. The source code for the web app is available in the [frontend](/frontend) directory.

### Running locally

#### Input files

The challenges depend on a specific puzzle input provided alongside each task.
This repository expects these files to be available in the location `/year/day_n/input.txt` however the creator has asked kindly that these files not be committed. As such you will need to provide your own puzzle inputs. From 2024 onwards, I have provided tests to run the publicly available example input through the solution. You may run the tests for any challenge to see the code in action without providing a puzzle input.

#### Rust

- **To run the solution:** navigate to the directory `/year/day_n/task_m/` and run the command `cargo run --release`.
- **To run the tests:** navigate to the directory `/year/day_n/task_m/` and run the command `cargo test`.

#### Go

- To run the solution, navigate to the directory `/year/day_n/` and run the command `go run task_m.go`.
