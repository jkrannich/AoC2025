pub fn run() {
    let input = include_str!("../inputs/day4input.txt");

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let columns = grid[0].len();

    let mut total_removed = 0;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..columns {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighboring_rolls = 0;

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
                            neighboring_rolls += 1;
                        }
                    }
                }
                if neighboring_rolls < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    println!("Total rolls removed: {total_removed}");
}