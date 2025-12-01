mod day1_part1;
mod day2;
mod day1_part2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map(String::as_str).unwrap_or("1");

    match day {
        "1" => day1_part1::run(),
        "2" => day2::run(),
        _ => println!("Unknown day: {day}"),
    }
}
