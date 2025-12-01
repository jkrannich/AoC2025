pub fn run() {
    let input = include_str!("../inputs/day1input.txt");

    let mut starting_dial = 50;
    let mut counter = 0;

    //lowest = 0
    //highest = 99
    //if below 0 -> start at 99
    //if above 99 -> start at 0
    //50 -> loop through every instruction
    //first input: L23
    //left -> substract 23 -> 50-23=27
    //second input: R14
    //right -> add 14 -> 27+14=41
    //etc.
    //important: number of times at 0

    //starting dial = 50
    //for line in input.lines()
    //let counter = 0;
    //let instruction = line
    //if line starts with L -> call substract(line without first letter)
    //if line starts with R -> call add(line without first letter)
    //if result = 0 -> increment counter by one
    //if not -> continue

    //how to solve dial?
    //dial = (dial + distance) modulo 100?

    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let distance: i32 = number.parse().unwrap();

        starting_dial = match direction {
            "L" => substract(distance, starting_dial),
            "R" => add(distance, starting_dial),
            _ => unreachable!(),
        };

        if starting_dial == 0 {
            counter += 1;
        }

        println!("{}", counter)
    }

    pub fn substract(distance: i32, current_dial: i32) -> i32 {
        (current_dial - distance).rem_euclid(100)
    }

    pub fn add(distance: i32, current_dial: i32) -> i32 {
        (current_dial + distance).rem_euclid(100)
    }
}