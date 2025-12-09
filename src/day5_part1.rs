pub fn run() {
    let input = include_str!("../inputs/day5input.txt");

    println!("{}", input);

    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(i64, i64)> = ranges
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect();

    println!("{:?}", ranges);

    let ingredients: Vec<i64> = ingredients
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| line.trim_end_matches(';'))
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    println!("{:?}", ingredients);

    //int fresh = 0;
    //maybe parse all ranges each number -> vec?
    //loop through each number in ingredients
    //for each number -> check if included in ranges
    //if yes -> fresh
    //if no -> rotten
    //fresh++
}