pub fn run() {
    let input = include_str!("../inputs/day3input.txt");

    let mut total_sum = 0;

    for line in input.lines() {
        let digits: Vec<u64> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut best_combination=0;

        for i in 0..digits.len() {
            for j in i+1..digits.len() {
                let a = digits[i];
                let b = digits[j];

                let candidate = a*10+b;

                if candidate > best_combination {
                    best_combination = candidate;
                }
            }
        }
        total_sum += best_combination;
        println!("{best_combination}");
    }
    println!("Total sum: {total_sum}");
}