use advent_of_code_2025::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut day_filter: Option<u32> = None;
    let mut part_filter: Option<u32> = None;

    for arg in args.iter().skip(1) {
        if let Ok(n) = arg.parse::<u32>() {
            if day_filter.is_none() {
                day_filter = Some(n);
            } else if part_filter.is_none() {
                part_filter = Some(n);
            }
        }
    }

    if let Some(day) = day_filter {
        let part_start = part_filter.unwrap_or(1);
        let part_end = part_filter.unwrap_or(2);

        for part in part_start..=part_end {
            run_test(day, part);
        }
    } else {
        println!("No day specified. Usage: cargo test -- <day> [part]");
    }
}

fn run_test(day: u32, part: u32) {
    let input_path = format!("inputs/day{:02}/example.txt", day);
    if std::path::Path::new(&input_path).exists() {
        let input = std::fs::read_to_string(&input_path).unwrap();
        days::run(day, part, &input);
    }
}
