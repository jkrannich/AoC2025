pub fn run() {
    let input = include_str!("../inputs/day9input.txt");

    let points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (xs, ys) = line.split_once(',').unwrap();
            let x = xs.trim().parse::<i64>().unwrap();
            let y = ys.trim().parse::<i64>().unwrap();
            (x, y)
        })
        .collect();

    let mut max_area: i64 = 0;

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        for j in i + 1..points.len() {
            let (x2, y2) = points[j];

            let width = (x1 - x2).abs() + 1;
            let height = (y1 - y2).abs() + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }
    println!("Largest rectanble area: {max_area}");
}