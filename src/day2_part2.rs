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

    //intialize result int = 0
    //for each (start, end) in ranges
    //for number in start..=end
    //convert number -> string
    //if string.length modulo 2 = 0 -> possible invalid id
    //for each entry in possible numbers list/vec
    //split number in half
    //if first half of digits == second half of digits -> invalid input
    //add to result int
    //return result int

    let mut result = 0;

    for (start, end) in &ranges {
        for number in *start..=*end {
            let s = number.to_string();

            if s.len() % 2 == 1 {
                continue;
            }

            let half = s.len()/2;
            let first_half = &s[..half];
            let second_half = &s[half..];

            if first_half == second_half {
                result += number;
            }
            println!("{}", result)
        }
    }
}