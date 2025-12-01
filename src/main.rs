mod day1;
mod day2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map(String::as_str).unwrap_or("1");

    match day {
        "1" => day1::run(),
        "2" => day2::run(),
        _ => println!("Unknown day: {day}"),
    }
}
