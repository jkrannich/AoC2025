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
    //n = string.legnth()
    //divide length into divisors (i.e length 9 -> 1,3)
    //3 -> divide string into 3 parts
    //compare if third 1 = third 2
    //if not equal -> continue
    //if equal -> add to result
    //return result

    let mut result = 0;

    for (start, end) in &ranges {
        for number in *start..=*end {
            let s = number.to_string();
            let n = s.len();

            let divisors = find_divisors(n);

            for divisor in divisors {
                let block_size = divisor;
                let pattern = &s[..block_size];

                let mut all_match = true;

                let num_blocks = n / block_size;

                if num_blocks < 2 {
                    continue;
                }

                for block_idx in 1..num_blocks {
                    let start = block_idx * block_size;
                    let end = start + block_size;

                    let chunk = &s[start..end];

                    if chunk != pattern {
                        all_match = false;
                        break;
                    }
                }

                if all_match {
                    result += number;
                    break;
                }
            }
            println!("{}", result)
        }
    }
}

fn find_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();

    for d in 1..n {
        if n % d == 0 {
            divisors.push(d);
        }
    }
    divisors
}