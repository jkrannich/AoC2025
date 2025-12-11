pub fn run() {
    let input = include_str!("../inputs/day6input.txt");

    let operator_idx = input
        .find(&['+', '*'])
        .unwrap();

    let num_str: &str = &input[0..operator_idx];
    let op_str: &str = &input[operator_idx..];
    let op_vec: Vec<char> = op_str.chars().filter(|ch| !ch.is_whitespace()).collect();
    let num_iter = num_str.lines();
    let mut sums: Vec<u64> = Vec::new();

    for line in num_iter {
        for (j, ch) in line.split_whitespace().enumerate() {
            let num: u64 = ch.parse().unwrap();

            if !sums.get(j).is_some() {
                sums.push(num);
                continue;
            }

            let op = op_vec.get(j).unwrap();
            match op {
                '+' => sums[j] += num,
                '*' => sums[j] *= num,
                _ => unreachable!(),
            }
        }
    }
    let result: u64 = sums.iter().sum();
    println!("{result}");
}