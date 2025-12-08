const K: usize = 12;
pub fn run() {
    let input = include_str!("../inputs/day3input.txt");

    let mut total_sum: i128 = 0;

    //same as before but now we need 12 digits to form best combination
    //new approach:
    //loop through entire input line and delete all numbers except the 12 largest ones

    for line in input.lines() {
        let digits: Vec<u8> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let best_digits = find_largest_k_digits(&digits, K);

        let mut value: u64 = 0;
        for d in best_digits {
            value = value * 10 + d as u64;
        }

        eprintln!("Best for line {} -> {}", line, value);

        total_sum += value as i128;
    }
    println!("Total sum: {total_sum}");
}

pub fn find_largest_k_digits(digits: &[u8], k: usize) -> Vec<u8> {
    let digit_amount = digits.len();
    assert!(k <= digit_amount);

    let mut to_remove = digit_amount-k;
    let mut stack: Vec<u8> = Vec::with_capacity(digit_amount);

    for &d in digits {
        while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);
    stack
}