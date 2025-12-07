#[derive(Debug)]
struct Interval(u64, u64);

fn parse_input(input: &str) -> (Vec<Interval>, Vec<u64>) {
    let mut fresh_intervals = vec![];
    let mut available = vec![];

    let mut p = 0;

    for l in input.lines() {
        if p == 0 {
            if l == "" {
                p = 1;
            } else {
                let split: Vec<&str> = l.split("-").collect();
                fresh_intervals.push(Interval(
                    split[0].parse().unwrap(),
                    split[1].parse().unwrap(),
                ));
            }
        } else {
            available.push(l.parse().unwrap());
        }
    }

    (fresh_intervals, available)
}

#[allow(unused)]
pub fn run(input: &str) {
    let (fresh_intervals, availables) = parse_input(input);

    let mut result = 0;
    for id in availables {
        for int in &fresh_intervals {
            if id >= int.0 && id <= int.1 {
                result += 1;
                break;
            }
        }
    }

    println!("Result: {}", result);
}
