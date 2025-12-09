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

    let mut merged: Vec<(i64, i64)> = Vec::new();

    for (start, end) in ranges {
        if let Some((last_start, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                *last_end = (*last_end).max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let mut total: i64 = 0;
    for (start, end) in merged {
        total += end - start + 1;
    }

    println!("Total fresh ingredients count: {total}");
}