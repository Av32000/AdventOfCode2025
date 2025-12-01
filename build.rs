use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

const DAYS_DIR: &str = "src/days";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("days_generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    println!("cargo:rerun-if-changed={}", DAYS_DIR);

    let mut days = Vec::new();
    if let Ok(entries) = fs::read_dir(DAYS_DIR) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(dirname) = path.file_name().and_then(|s| s.to_str()) {
                        if let Ok(day_num) = dirname.parse::<u32>() {
                            days.push(day_num);
                        }
                    }
                }
            }
        }
    }

    days.sort();

    let cwd = env::current_dir().unwrap();
    let days_dir_abs = cwd.join(DAYS_DIR);

    for day_num in &days {
        let day_mod = format!("day{:02}", day_num);
        let day_path = days_dir_abs.join(format!("{:02}", day_num));

        writeln!(f, "pub mod {} {{", day_mod).unwrap();

        let part1_path = day_path.join("part1.rs");
        if part1_path.exists() {
            writeln!(f, "    #[path = \"{}\"]", part1_path.display()).unwrap();
            writeln!(f, "    pub mod part1;").unwrap();
        }

        let part2_path = day_path.join("part2.rs");
        if part2_path.exists() {
            writeln!(f, "    #[path = \"{}\"]", part2_path.display()).unwrap();
            writeln!(f, "    pub mod part2;").unwrap();
        }

        writeln!(f, "}}").unwrap();
    }

    writeln!(f, "").unwrap();
    writeln!(f, "pub fn run(day: u32, part: u32, input: &str) {{").unwrap();
    writeln!(f, "    match (day, part) {{").unwrap();
    for day_num in &days {
        let day_mod = format!("day{:02}", day_num);
        let day_path = days_dir_abs.join(format!("{:02}", day_num));

        if day_path.join("part1.rs").exists() {
            writeln!(
                f,
                "        ({}, 1) => {}::part1::run(input),",
                day_num, day_mod
            )
            .unwrap();
        }
        if day_path.join("part2.rs").exists() {
            writeln!(
                f,
                "        ({}, 2) => {}::part2::run(input),",
                day_num, day_mod
            )
            .unwrap();
        }
    }
    writeln!(
        f,
        "        _ => println!(\"Day {{}} Part {{}} not implemented\", day, part),"
    )
    .unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}
