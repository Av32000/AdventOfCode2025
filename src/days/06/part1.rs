#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    number: Vec<i64>,
    op: Operation,
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn parse_inputs(input: &str) -> Vec<Problem> {
    let mut result = Vec::new();

    let mut current_str = String::new();
    let mut current_p = 0;
    for (i, line) in input.lines().enumerate() {
        for c in line.chars() {
            if i == 0 {
                if DIGITS.contains(&c) {
                    current_str += &c.to_string();
                } else if !current_str.is_empty() {
                    result.push(Problem {
                        number: vec![current_str.parse().unwrap()],
                        op: Operation::Add,
                    });
                    current_str = String::new();
                }
            } else if i < input.lines().count() - 1 {
                if DIGITS.contains(&c) {
                    current_str += &c.to_string();
                } else if !current_str.is_empty() {
                    result[current_p].number.push(current_str.parse().unwrap());
                    current_str = String::new();
                    current_p += 1;
                }
            } else {
                if c == '+' {
                    result[current_p].op = Operation::Add;
                    current_p += 1;
                } else if c == '*' {
                    result[current_p].op = Operation::Multiply;
                    current_p += 1;
                }
            }
        }
        if !current_str.is_empty() {
            if i == 0 {
                result.push(Problem {
                    number: vec![current_str.parse().unwrap()],
                    op: Operation::Add,
                });
            } else if i < input.lines().count() - 1 {
                result[current_p].number.push(current_str.parse().unwrap());
            } else {
                result[current_p].op = Operation::Multiply;
            }
        }

        current_str = String::new();
        current_p = 0;
    }

    result
}

#[allow(unused)]
pub fn run(input: &str) {
    let mut problems = parse_inputs(input);

    let mut result = 0;
    for problem in problems {
        result += match problem.op {
            Operation::Add => {
                let mut sum = 0;
                for n in problem.number {
                    sum += n;
                }
                sum
            }
            Operation::Multiply => {
                let mut product = 1;
                for n in problem.number {
                    product *= n;
                }
                product
            }
        };
    }

    println!("Result: {}", result);
}
