#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    number: Vec<i64>,
    op: Operation,
}

fn parse_inputs(input: &str) -> Vec<Problem> {
    let mut columns =
        vec![vec![' '; input.lines().count() - 1]; input.lines().nth(0).unwrap().chars().count()];
    let mut ops = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if i < input.lines().count() - 1 {
                columns[j][i] = c;
            } else {
                match c {
                    '+' => {
                        ops.push(Operation::Add);
                    }
                    '*' => {
                        ops.push(Operation::Multiply);
                    }
                    _ => {}
                }
            }
        }
    }

    let mut problems = Vec::new();
    let mut current_p = 0;
    let mut current_numbers = Vec::new();
    for c in columns {
        let concat = c.iter().collect::<String>();
        let concat = concat.trim();
        if concat.is_empty() {
            problems.push(Problem {
                number: current_numbers,
                op: ops[current_p].clone(),
            });
            current_numbers = Vec::new();
            current_p += 1;
        } else {
            current_numbers.push(concat.parse().unwrap());
        }
    }

    if current_numbers.len() > 0 {
        problems.push(Problem {
            number: current_numbers,
            op: ops.pop().unwrap(),
        });
    }

    problems
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
