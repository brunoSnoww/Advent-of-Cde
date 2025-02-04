use std::{collections::HashSet, hash::Hash};

fn next_char(c: char) -> char {
    match c {
        '^' => '>',
        '>' => 'V',
        'V' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

fn next_move(x: i32, y: i32, c: char) -> (i32, i32) {
    match c {
        '^' => (x - 1, y),
        '>' => (x, y + 1),
        'V' => (x + 1, y),
        '<' => (x, y - 1),
        _ => unreachable!(),
    }
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut c = '^';
    let grid: Vec<Vec<char>> = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let p: Vec<_> = l.chars().collect();
            p.iter().enumerate().for_each(|(j, c)| {
                if *c == '^' {
                    x = i;
                    y = j;
                }
            });
            p
        })
        .collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut hs = HashSet::new();
    hs.insert((x, y));
    loop {
        let (xx, yy) = next_move(x as i32, y as i32, c);
        if xx >= 0 && xx < n as i32 && yy >= 0 && yy < m as i32 {
            if grid[xx as usize][yy as usize] == '#' {
                c = next_char(c);
            } else {
                x = xx as usize;
                y = yy as usize;
                hs.insert((x, y));
            }
        } else {
            break;
        }
    }
    println!("{}", hs.len());
}
