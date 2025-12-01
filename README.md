# Advent of Code 2025 - Rust

My solutions for the [Advent of Code 2025](https://adventofcode.com/2025) in Rust.

## Setup

This project uses a custom build system to automatically discover and run solutions.

### Structure
- **Solutions**: `src/days/XX/partX.rs` (e.g., `src/days/01/part1.rs`)
- **Inputs**: `inputs/dayXX/input.txt` (real input) and `inputs/dayXX/example.txt` (test input)

### Usage

**Run Solution (Real Input)**
```bash
cargo run -- <day> <part>
# Example: cargo run -- 1 1
```

**Run Test (Example Input)**
```bash
cargo test -- <day> <part>
# Example: cargo test -- 1 1
```

## Solutions
| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 01](src/days/01) | [Part 1](src/days/01/part1.rs) | [Part 2](src/days/01/part2.rs) |
