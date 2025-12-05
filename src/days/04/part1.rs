fn parse_inputs(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
        .collect()
}

const ADJ_COORDS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[allow(unused)]
pub fn run(input: &str) {
    let map = parse_inputs(input);

    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 1 {
                let mut adj_result = 0;
                for c in ADJ_COORDS {
                    let c_x = j as i32 + c.0 as i32;
                    let c_y = i as i32 + c.1 as i32;

                    if c_x < 0 || c_y < 0 {
                        continue;
                    }

                    if map
                        .get(c_y as usize)
                        .unwrap_or(&vec![0])
                        .get(c_x as usize)
                        .unwrap_or(&0)
                        == &1
                    {
                        adj_result += 1;
                    }
                }
                if adj_result < 4 {
                    result += 1;
                }
            }
        }
    }

    println!("Result: {}", result);
}
