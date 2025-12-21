use std::collections::VecDeque;

pub fn run() {
    let input = include_str!("../inputs/day7input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let h = lines.len();
    let w = lines[0].len();

    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

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

    let mut cur = vec![0u128; w];
    cur[sx] = 1;

    let mut answer: u128 = 0;

    for y in sy..h {
        let mut q = VecDeque::new();
        for x in 0..w {
            if cur[x] > 0 && grid[y][x] == '^' {
                q.push_back(x);
            }
        }

        while let Some(x) = q.pop_front() {
            if cur[x] == 0 {
                continue;
            }
            if grid[y][x] != '^' {
                continue;
            }

            let ways = cur[x];
            cur[x] = 0;

            if x > 0 {
                let nx = x - 1;
                let before = cur[nx];
                cur[nx] = cur[nx].saturating_add(ways);
                if grid[y][nx] == '^' && before == 0 && cur[nx] > 0 {
                    q.push_back(nx);
                }
            }
            if x + 1 < w {
                let nx = x + 1;
                let before = cur[nx];
                cur[nx] = cur[nx].saturating_add(ways);
                if grid[y][nx] == '^' && before == 0 && cur[nx] > 0 {
                    q.push_back(nx);
                }
            }
        }

        if y == h - 1 {
            for x in 0..w {
                if grid[y][x] == '.' || grid[y][x] == 'S' {
                    answer = answer.saturating_add(cur[x]);
                }
            }
        } else {
            let mut next = vec![0u128; w];
            for x in 0..w {
                if grid[y][x] == '.' || grid[y][x] == 'S' {
                    next[x] = next[x].saturating_add(cur[x]);
                }
            }
            cur = next;
        }
    }

    println!("{}", answer);
}
