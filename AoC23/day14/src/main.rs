use std::collections::{HashMap, HashSet};

struct Grid {
    pub cells: Vec<Vec<char>>,
    N: usize,
    M: usize,
}

impl Grid {
    pub fn tick_north(&mut self) {
        let mut next = self.cells.clone();
        for i in 1..self.N {
            for j in 0..self.M {
                if self.cells[i][j] != 'O' {
                    continue;
                }
                let mut ii = i - 1;
                let mut hitBlock = false;
                let mut hitWall = false;
                loop {
                    if (next[ii][j] == 'O' || next[ii][j] == '#') {
                        if ii != i - 1 {
                            hitBlock = true;
                        }
                        break;
                    }
                    if ii == 0 {
                        if next[ii][j] == '.' {
                            hitWall = true;
                        }
                        break;
                    }
                    ii -= 1;
                }
                if hitWall {
                    next[ii][j] = 'O';
                    next[i][j] = '.';
                }
                if hitBlock {
                    next[ii + 1][j] = 'O';
                    next[i][j] = '.';
                }
            }
        }
        self.cells = next;
    }
    pub fn tick_south(&mut self) {
        let mut next = self.cells.clone();
        for i in (0..self.N - 1).rev() {
            for j in 0..self.M {
                if self.cells[i][j] != 'O' {
                    continue;
                }
                let mut ii = i + 1;
                let mut hitBlock = false;
                let mut hitWall = false;
                loop {
                    if (next[ii][j] == 'O' || next[ii][j] == '#') {
                        if ii != i + 1 {
                            hitBlock = true;
                        }
                        break;
                    }
                    if ii == self.N - 1 {
                        if next[ii][j] == '.' {
                            hitWall = true;
                        }
                        break;
                    }
                    ii += 1;
                }
                if hitWall {
                    next[ii][j] = 'O';
                    next[i][j] = '.';
                }
                if hitBlock {
                    next[ii - 1][j] = 'O';
                    next[i][j] = '.';
                }
            }
        }
        self.cells = next;
    }
    pub fn tick_east(&mut self) {
        let mut next = self.cells.clone();
        for j in (0..self.M - 1).rev() {
            for i in 0..self.N {
                if self.cells[i][j] != 'O' {
                    continue;
                }
                let mut jj = j + 1;
                let mut hitBlock = false;
                let mut hitWall = false;
                loop {
                    if (next[i][jj] == 'O' || next[i][jj] == '#') {
                        if jj != j + 1 {
                            hitBlock = true;
                        }
                        break;
                    }
                    if jj == self.M - 1 {
                        if next[i][jj] == '.' {
                            hitWall = true;
                        }
                        break;
                    }
                    jj += 1;
                }
                if hitWall {
                    next[i][jj] = 'O';
                    next[i][j] = '.';
                }
                if hitBlock {
                    next[i][jj - 1] = 'O';
                    next[i][j] = '.';
                }
            }
        }
        self.cells = next;
    }
    pub fn tick_west(&mut self) {
        let mut next = self.cells.clone();
        for j in (1..self.M) {
            for i in 0..self.N {
                if self.cells[i][j] != 'O' {
                    continue;
                }
                let mut jj = j - 1;
                let mut hitBlock = false;
                let mut hitWall = false;
                loop {
                    if (next[i][jj] == 'O' || next[i][jj] == '#') {
                        if jj != j - 1 {
                            hitBlock = true;
                        }
                        break;
                    }
                    if jj == 0 {
                        if next[i][jj] == '.' {
                            hitWall = true;
                        }
                        break;
                    }
                    jj -= 1;
                }
                if hitWall {
                    next[i][jj] = 'O';
                    next[i][j] = '.';
                }
                if hitBlock {
                    next[i][jj + 1] = 'O';
                    next[i][j] = '.';
                }
            }
        }
        self.cells = next;
    }

    pub fn tick(&mut self) {
        self.tick_north();
        self.tick_west();
        self.tick_south();
        self.tick_east();
    }

    pub fn load(&self) -> i32 {
        let mut ans = 0;
        for (i, line) in self.cells.iter().enumerate() {
            let count = line.iter().filter(|x| **x == 'O').count() as i32;
            ans += (count * ((self.N - i) as i32));
        }
        ans
    }
}

fn main() {
    let file = include_str!("./input15.txt");
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let N = grid.len();
    let M = grid[0].len();
    let mut g1 = Grid {
        cells: grid.clone(),
        N,
        M,
    };
    let mut g2 = Grid {
        cells: grid.clone(),
        N,
        M,
    };
    g1.tick_north();
    println!("Part1: {}", g1.load());
    for _ in 0..1000 {
        g2.tick();
    }
    println!("Part2: {}", g2.load());
}
