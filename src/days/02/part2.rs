struct Interval(u64, u64);

fn parse_inputs(input: &str) -> Vec<Interval> {
    let mut result = vec![];
    for interval in input.lines().nth(0).unwrap().to_string().split(",") {
        result.push(Interval(
            interval.split("-").nth(0).unwrap().parse().unwrap(),
            interval.split("-").nth(1).unwrap().parse().unwrap(),
        ));
    }
    result
}

fn is_valid(n: u64) -> bool {
    let s = n.to_string();
    let s_len = s.len();

    for i in 1..(s_len / 2 + 1) {
        if s_len % i == 0 {
            let sub = s[..i].to_string();
            if sub.repeat(s_len / i) == s {
                return false;
            }
        }
    }

    return true;
}

#[allow(unused)]
pub fn run(input: &str) {
    let intervals = parse_inputs(input);
    let mut result = 0;
    for i in intervals {
        for n in i.0..(i.1 + 1) {
            if !is_valid(n) {
                result += n;
            }
        }
    }

    println!("Result: {}", result);
}
