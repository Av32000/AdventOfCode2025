#[derive(Debug)]
enum MoveType {
    Left,
    Right,
}

struct Move(MoveType, i32);

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
        let coef = match m.0 {
            MoveType::Left => -1,
            MoveType::Right => 1,
        };

        current_pos += (coef * m.1);
        current_pos %= 100;

        if current_pos < 0 {
            current_pos = 100 + current_pos
        }

        if current_pos == 0 {
            result += 1;
        }
    }

    println!("Result: {}, Final Position: {}", result, current_pos);
}
