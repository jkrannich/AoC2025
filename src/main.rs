mod day1_part1;
mod day2_part1;
mod day1_part2;
mod day2_part2;
mod day3_part1;
mod day3_part2;
mod day4_part1;
mod day4_part2;
mod day5_part1;
mod day5_part2;
mod day9_part1;
mod day6_part1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map(String::as_str).unwrap_or("1");

    match day {
        "1.1" => day1_part1::run(),
        "1.2" => day1_part2::run(),
        "2.1" => day2_part1::run(),
        "2.2" => day2_part2::run(),
        "3.1" => day3_part1::run(),
        "3.2" => day3_part2::run(),
        "4.1" => day4_part1::run(),
        "4.2" => day4_part2::run(),
        "5.1" => day5_part1::run(),
        "5.2" => day5_part2::run(),
        "6.1" => day6_part1::run(),
        "9.1" => day9_part1::run(),
        _ => println!("Unknown day: {day}"),
    }
}
