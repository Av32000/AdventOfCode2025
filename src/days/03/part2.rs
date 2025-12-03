#[allow(unused)]
pub fn run(input: &str) {
    let mut result: u64 = 0;
    for line in input.lines() {
        let mut max_arr = vec!['0'; 12];

        let line_len = line.len();

        for (i, c) in line.chars().enumerate() {
            let mut found = false;
            for (j, current) in max_arr.clone().iter().enumerate() {
                if found {
                    max_arr[j] = '0';
                } else if c > *current && (line_len - i) >= (12 - j) {
                    max_arr[j] = c;
                    found = true;
                }
            }
        }

        let mut result_string = String::new();
        for c in max_arr {
            result_string += &c.to_string();
        }

        result += result_string.parse::<u64>().unwrap();
    }

    println!("Result: {}", result);
}
