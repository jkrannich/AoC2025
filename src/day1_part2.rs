pub fn run() {
    //new twist: we now want to count every time we cross 0 too
    //how to do it?
    //loop distance amount of times
    //each iteration = one move to left/right
    //each move -> currentdial++
    //check if current dial == 0 -> if yes counter++
    let input = include_str!("../inputs/day1input.txt");

    let mut starting_dial: i32 = 50;
    let mut counter = 0;

    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let distance: i32 = number.parse().unwrap();

        for _ in 0..distance {
            match direction {
                "L" => starting_dial -= 1,
                "R" => starting_dial += 1,
                _ => unreachable!(),
            };

            starting_dial = starting_dial.rem_euclid(100);

            if starting_dial == 0 {
                counter += 1;
            }

            println!("{counter}");
        }

        println!("{}", counter)
    }
}