#[derive(Debug)]
enum MoveType {
    Left,
    Right,
}

struct Move(MoveType, i32);

fn apply(pos: i32, m: &Move) -> (i32, i32) {
    let mut new_pos = pos;
    let mut result = 0;

    let coef = match m.0 {
        MoveType::Left => -1,
        MoveType::Right => 1,
    };

    if m.1 == 0 {
        return (pos, 0);
    }

    for _ in 0..(m.1.abs()) {
        new_pos += coef;

        if new_pos == -1 {
            new_pos = 99;
        }
        if new_pos == 100 {
            new_pos = 0;
        }

        if new_pos == 0 {
            result += 1;
        }
    }

    (new_pos, result)
}

fn parse_input(input: &str) -> Vec<Move> {
    let mut moves = vec![];
    for line in input.lines() {
        let move_type = match line.chars().nth(0).unwrap_or(' ') {
            'L' => MoveType::Left,
            'R' => MoveType::Right,
            _ => {
                continue;
            }
        };

        let move_count: i32 = line.to_string()[1..].parse().unwrap_or(0);

        moves.push(Move(move_type, move_count));
    }
    moves
}

#[allow(unused)]
pub fn run(input: &str) {
    let moves = parse_input(input);

    let mut result = 0;
    let mut current_pos = 50;

    for m in moves {
        let (new_pos, new_res) = apply(current_pos, &m);
        current_pos = new_pos;
        result += new_res;
    }

    println!("Result: {}, Final Position: {}", result, current_pos);
}
