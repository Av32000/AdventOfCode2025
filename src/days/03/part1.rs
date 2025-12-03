#[allow(unused)]
pub fn run(input: &str) {
    let mut result: u64 = 0;
    for line in input.lines() {
        let mut max1 = '0';
        let mut max2 = '0';

        let line_len = line.len();

        for (i, c) in line.chars().enumerate() {
            if c > max1 && i < line_len - 1 {
                max1 = c;
                max2 = '0';
            } else if c > max2 {
                max2 = c;
            }
        }

        result += (format!("{}{}", max1, max2)).parse::<u64>().unwrap();
    }

    println!("Result: {}", result);
}
