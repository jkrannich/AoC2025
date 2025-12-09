pub fn run() {
    let input = include_str!("../inputs/day5input.txt");

    let (ranges, _nums) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(i64, i64)> = ranges
        .lines()
        .map(|line| {
            let (a,b) = line.split_once('-').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect();

    //we need to count all numbers in ranges
    //now iterate through each range
    //add all numbers from start -> end into a vec?
    //count numbers in vec at end
    //problem: memory issues due to large ranges
    //smarter: check overlaps and only include them once
    //how: sort vec by start
    //merge overlapping ranges?
    //then count


    ranges.sort_by_key(|&(start, _end)| start);

    for (s, e) in ranges.iter().take(10) {
        println!("{s}-{e}");
    }
}