# 🎄 Advent of Code 🎄

Solutions to [Advent of Code](https://adventofcode.com/) programming challenges for various years.

## How to run the programs

### Web app

The simplest way to use the programs is to visit the web app [hosted here](https://aoc.oliver-bilbie.co.uk/) which provides an interface for running the solutions directly from a web browser using WebAssembly. The source code for the web app is available in the [frontend](/frontend) directory.

Rust solutions have been compiled directly into WASM, so will be lightweight and fast.
Go solutions are built using the [TinyGo](https://tinygo.org/) runtime, which is lightweight but has fewer optimizations than the official Go runtime.
Python is an interpreted language, so cannot be directly compiled to WASM. [Pyodide](https://pyodide.org/en/stable/) is a Python interpreter which targets WASM, and is used here to run Python solutions.

### Running locally

#### Input files

The challenges depend on a specific puzzle input provided alongside each task.
This repository expects these files to be available in the location `/year/day_n/input.txt` however the creator has asked that these files not be committed. As such you will need to provide your own puzzle inputs. For most solutions I have provided tests to solve the publicly available example input. You may run the tests for any challenge to see the code in action without providing your own puzzle input.

#### Rust

- **To run the solution:** navigate to the directory `/year/day_n/task_m/` and run the command `cargo run --release`.
- **To run the tests:** navigate to the directory `/year/day_n/task_m/` and run the command `cargo test`.

#### Go

- **To run the solution:** navigate to the directory `/year/day_n/task_m/` and run the command `go run .`.
- **To run the tests:** navigate to the directory `/year/day_n/task_m/` and run the command `go test`.

#### Python

- **To run the solution:** navigate to the directory `/year/day_n/task_m/` and run the command `python main.py`.
- **To run the tests:** navigate to the directory `/year/day_n/task_m/` and run the command `python -m unittest`.

## Adding new solutions

A TUI app for quickly bootstrapping a new solution is provided, and can be started by running `make new` from the root directory.

## Building the web app

This repository contains all of the necessary scripts to deploy the web app to AWS.
To do so, provide a `terraform/terraform.tfvars` file containing the following:

```
region = "us-east-1"
app_name = "aoc-solver"
base_domain = "my-domain.com"
full_domain = "aoc.my-domain.com"
cert_arn = "arn:aws:acm:us-east-1:012345678912:certificate/b2124057-6e1e-4221-a127-66ff3a83a1e5"
```

and run the command `make` from the root directory.
