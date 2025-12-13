pub fn run() {
    let input = include_str!("../inputs/day6input.txt");

    let mut rows: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    for r in &mut rows {
        if r.len() < width {
            r.resize(width, ' ');
        }
    }

    let op_row = rows
        .iter()
        .position(|r| r.iter().any(|&ch| ch == '+' || ch == '*'))
        .expect("no operator row found");

    // 3) Determine separator columns (all spaces in every row)
    let mut is_sep = vec![true; width];
    for c in 0..width {
        for r in 0..rows.len() {
            if rows[r][c] != ' ' {
                is_sep[c] = false;
                break;
            }
        }
    }

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    while c < width {
        while c < width && is_sep[c] {
            c += 1;
        }
        if c >= width {
            break;
        }
        let start = c;
        while c < width && !is_sep[c] {
            c += 1;
        }
        let end = c;
        blocks.push((start, end));
    }

    let parse_col_num = |rows: &Vec<Vec<char>>, col: usize, op_row: usize| -> Option<u64> {
        let mut val: u64 = 0;
        let mut seen = false;
        for r in 0..op_row {
            let ch = rows[r][col];
            if ch.is_ascii_digit() {
                seen = true;
                val = val * 10 + (ch as u8 - b'0') as u64;
            }
        }
        seen.then_some(val)
    };

    let mut total: u64 = 0;

    for (start, end) in blocks {
        let mut op: Option<char> = None;
        for c in start..end {
            let ch = rows[op_row][c];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("block without operator");

        let mut acc: Option<u64> = None;
        for col in (start..end).rev() {
            if let Some(x) = parse_col_num(&rows, col, op_row) {
                acc = Some(match (acc, op) {
                    (None, _) => x,
                    (Some(a), '+') => a + x,
                    (Some(a), '*') => a * x,
                    _ => unreachable!(),
                });
            }
        }

        total += acc.unwrap_or(0);
    }

    println!("{total}");
}