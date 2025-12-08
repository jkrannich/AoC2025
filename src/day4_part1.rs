pub fn run() {
    let input = include_str!("../inputs/day4input.txt");

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let columns = grid[0].len();

    let mut accessible = 0;

    for r in 0..rows {
        for c in 0..columns {
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbor_rolls = 0;

            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let neighbor_row = r as i32 + dr;
                    let neighbor_col = c as i32 + dc;

                    if neighbor_row < 0 || neighbor_col < 0 || neighbor_row >= rows as i32 || neighbor_col >= columns as i32 {
                        continue;
                    }

                    if grid[neighbor_row as usize][neighbor_col as usize] == '@' {
                        neighbor_rolls += 1;
                    }
                }
            }
            if neighbor_rolls < 4 {
                accessible += 1;
            }
        }
    }
    println!("{}", accessible);
}