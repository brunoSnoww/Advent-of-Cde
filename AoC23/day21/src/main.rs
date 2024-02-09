use std::collections::HashSet;

fn f(x: i32, n: i32, m: i32, c: &HashSet<(i32, i32, (i32, i32))>, hs: &HashSet<(i32, i32)>) -> i64 {
    let mut current_state = c.clone();
    for _ in 0..x {
        let mut next_state = HashSet::new();
        for (i, j, (block_row, block_col)) in current_state.clone() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut ni = i + dx;
                let mut nj = j + dy;
                let nbi = if ni < 0 {
                    block_row - 1
                } else if ni == n {
                    block_row + 1
                } else {
                    block_row
                };
                let nbj = if nj < 0 {
                    block_col - 1
                } else if nj == m {
                    block_col + 1
                } else {
                    block_col
                };
                ni = (ni + n) % n;
                nj = (nj + m) % m;

                if !hs.contains(&(ni, nj)) {
                    next_state.insert((ni, nj, (nbi, nbj)));
                }
            }
        }
        current_state = next_state;
    }

    current_state.len() as i64
}

fn main() {
    let mut hs = HashSet::new();
    let mut ini = (0, 0, (0, 0));
    let mut n = 0;
    let mut m = 0;
    include_str!("./input21.txt")
        .lines()
        .enumerate()
        .for_each(|(i, l)| {
            m = l.len() as i32;
            n += 1;
            l.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    hs.insert((i as i32, j as i32));
                }
                if c == 'S' {
                    ini = (i as i32, j as i32, (0, 0));
                }
            })
        });

    println!("{} {}", n, m);
    let mut current_state = HashSet::from([ini]);
    let y1 = f(65, n, m, &current_state, &hs);
    let y2 = f(65 + 131, n, m, &current_state, &hs);
    let y3 = f(65 + 131 * 2, n, m, &current_state, &hs);
    println!("{} {} {}", y1, y2, y3);
}
