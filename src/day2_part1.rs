pub fn run() {
    let input = include_str!("../inputs/day2input.txt");

    let ranges: Vec<(u64, u64)> = input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|chunk| {
            let mut parts = chunk.split('-');

            let start = parts.next().unwrap().parse::<u64>().unwrap();
            let end = parts.next().unwrap().parse::<u64>().unwrap();

            (start, end)
        })
        .collect();

    println!("{ranges:#?}");
}