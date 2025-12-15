use std::collections::VecDeque;

pub fn run() {
    let input = include_str!("../inputs/day7input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();

    let mut sx = None;
    let mut sy = None;
    'outer: for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 'S' {
                sx = Some(x);
                sy = Some(y);
                break 'outer;
            }
        }
    }
    let (sx, sy) = (sx.expect("no S found"), sy.expect("no S found"));

    let mut active = vec![false; w];
    active[sx] = true;

    let mut splits: u64 = 0;

    for y in (sy + 1)..h {
        let mut q = VecDeque::new();
        for x in 0..w {
            if active[x] && grid[y][x] == '^' {
                q.push_back(x);
            }
        }

        while let Some(x) = q.pop_front() {
            if !active[x] || grid[y][x] != '^' {
                continue;
            }

            splits += 1;

            active[x] = false;

            if x > 0 {
                if !active[x - 1] {
                    active[x - 1] = true;
                    if grid[y][x - 1] == '^' {
                        q.push_back(x - 1);
                    }
                }
            }
            if x + 1 < w {
                if !active[x + 1] {
                    active[x + 1] = true;
                    if grid[y][x + 1] == '^' {
                        q.push_back(x + 1);
                    }
                }
            }
        }
    }
    println!("{splits}");
}
