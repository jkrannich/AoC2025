pub fn run() {
    let input = include_str!("../inputs/day3input.txt");

    for line in input.lines() {
        let digits: Vec<u64> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut total_sum = 0;
        let mut best_combination=0;

        for &a in &digits {
            for &b in &digits {
                if a == b {
                    continue;
                }

                let candidate = concat(a, b);

                if candidate > best_combination {
                    best_combination = candidate;
                    total_sum += best_combination;
                }
            }
        }
        println!("Total sum: {total_sum}");
        println!("{best_combination}");
    }
}

pub fn concat(a: u64, b: u64) -> u64 {
    let s = format!("{a}{b}");
    s.parse::<u64>().unwrap()
}