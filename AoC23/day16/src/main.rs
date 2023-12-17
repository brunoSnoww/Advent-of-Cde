use std::collections::{HashSet, VecDeque};
use std::io::Write;
use termion::{clear, cursor};

fn print_grid(grid: &Vec<Vec<char>>, hs: &HashSet<(i32, i32)>) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' && hs.contains(&(i as i32, j as i32)) {
                print!("#");
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
    // Flush the output buffer
    if let Err(e) = std::io::stdout().flush() {
        eprintln!("Error flushing stdout: {}", e);
    }
}

fn cost(grid: &Vec<Vec<char>>, xi: i32, yi: i32, dir: i32) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((xi, yi, dir));
    let mut hs: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut hs2: HashSet<(i32, i32)> = HashSet::new();
    let n = grid.len();
    let m = grid[0].len();
    while !q.is_empty() {
        let (x, y, dir) = q.pop_front().unwrap();
        if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 {
            continue;
        }
        hs2.insert((x, y));
        if hs.contains(&(x, y, dir)) {
            continue;
        }
        hs.insert((x, y, dir));

        let c = grid[x as usize][y as usize];
        if c == '.' {
            let dx = if dir == UP {
                -1
            } else if dir == DOWN {
                1
            } else {
                0
            };
            let dy = if dir == RIGHT {
                1
            } else if dir == LEFT {
                -1
            } else {
                0
            };
            q.push_back((x + dx, y + dy, dir));
        }
        if c == '/' || c == '\\' {
            if dir == UP && c == '/' {
                q.push_back((x, y + 1, RIGHT));
            }
            if dir == DOWN && c == '/' {
                q.push_back((x, y - 1, LEFT));
            }
            if dir == UP && c == '\\' {
                q.push_back((x, y - 1, LEFT));
            }
            if dir == DOWN && c == '\\' {
                q.push_back((x, y + 1, RIGHT));
            }
            if dir == RIGHT && c == '/' {
                q.push_back((x - 1, y, UP));
            }
            if dir == LEFT && c == '/' {
                q.push_back((x + 1, y, DOWN));
            }
            if dir == RIGHT && c == '\\' {
                q.push_back((x + 1, y, DOWN));
            }
            if dir == LEFT && c == '\\' {
                q.push_back((x - 1, y, UP));
            }
        }
        if c == '-' {
            if dir == RIGHT {
                q.push_back((x, y + 1, RIGHT));
            }
            if dir == LEFT {
                q.push_back((x, y - 1, LEFT));
            }
            if dir == UP || dir == DOWN {
                q.push_back((x, y + 1, RIGHT));
                q.push_back((x, y - 1, LEFT));
            }
        }
        if c == '|' {
            if dir == RIGHT || dir == LEFT {
                q.push_back((x + 1, y, DOWN));
                q.push_back((x - 1, y, UP));
            } else {
                let dx = if dir == DOWN { 1 } else { -1 };
                q.push_back((x + dx, y, dir));
            }
        }
        //thread::sleep(Duration::from_millis(1000)); // Sleep for 500 milliseconds
        //print_grid(&grid, &hs);
    }
    hs2.len() as i32
}

const RIGHT: i32 = 1;
const LEFT: i32 = 2;
const UP: i32 = 3;
const DOWN: i32 = 4;

fn main() {
    let grid: Vec<Vec<char>> = include_str!("./input16.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let p1 = cost(&grid, 0, 0, RIGHT);
    println!("Part1: {}", p1);

    let mut p2 = -1;
    let n = grid.len();
    let m = grid[0].len();
    for j in 0..m {
        p2 = p2.max(cost(&grid, 0, j as i32, DOWN));
        p2 = p2.max(cost(&grid, (n - 1) as i32, j as i32, UP));
    }
    for i in 0..n {
        p2 = p2.max(cost(&grid, i as i32, 0, RIGHT));
        p2 = p2.max(cost(&grid, i as i32, (m - 1) as i32, LEFT));
    }

    println!("Part2: {}", p2);
}
