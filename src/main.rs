use advent_of_code_2025::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <day> <part>", args[0]);
        std::process::exit(1);
    }

    let day: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: <day> must be a number");
            std::process::exit(1);
        }
    };

    let part: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: <part> must be a number");
            std::process::exit(1);
        }
    };

    let input_path = format!("inputs/day{:02}/input.txt", day);
    let input = match std::fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file {}: {}", input_path, e);
            std::process::exit(1);
        }
    };

    days::run(day, part, &input);
}
