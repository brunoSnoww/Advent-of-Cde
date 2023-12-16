use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    pub i: i32,
    pub j: i32,
}

fn neighbours(p: &Point, c: char) -> Vec<Point> {
    match c {
        '-' => vec![Point { i: p.i, j: p.j - 1 }, Point { i: p.i, j: p.j + 1 }],
        '|' => vec![Point { i: p.i + 1, j: p.j }, Point { i: p.i - 1, j: p.j }],
        'L' => vec![Point { i: p.i, j: p.j + 1 }, Point { i: p.i - 1, j: p.j }],
        'J' => vec![Point { i: p.i, j: p.j - 1 }, Point { i: p.i - 1, j: p.j }],
        '7' => vec![Point { i: p.i + 1, j: p.j }, Point { i: p.i, j: p.j - 1 }],
        'F' => vec![Point { i: p.i + 1, j: p.j }, Point { i: p.i, j: p.j + 1 }],
        'S' => vec![
            Point { i: p.i + 1, j: p.j },
            Point { i: p.i, j: p.j + 1 },
            Point { i: p.i - 1, j: p.j },
            Point { i: p.i, j: p.j - 1 },
        ],
        _ => unreachable!(),
    }
}

fn dfs(g: &HashMap<Point, Vec<Point>>, visited: &mut HashSet<Point>, current: &Point, d: &mut i32) {
    visited.insert(current.clone());
    if let Some(neighbours) = g.get(current) {
        for pp in neighbours.iter() {
            if !visited.contains(pp) {
                *d += 1;
                dfs(g, visited, pp, d);
            }
        }
    }
}

fn main() {
    let input = include_str!("./input10.txt");
    let mut g: HashMap<Point, Vec<Point>> = HashMap::new();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let N = grid.len() as i32;
    let M = grid[0].len() as i32;
    let mut start: Point = Point { i: -1, j: -1 };
    let mut totalDots = 0;
    input.lines().enumerate().for_each(|(i, l)| {
        let v: Vec<char> = l.chars().collect();
        v.iter().enumerate().for_each(|(j, c)| {
            if *c == '.' {
                totalDots += 1;
            } else {
                let point = Point {
                    i: i as i32,
                    j: j as i32,
                };
                if *c == 'S' {
                    start = point.clone();
                }
                let current = g.entry(point.clone()).or_insert_with(Vec::new);
                neighbours(&point, *c).iter().for_each(|pp| {
                    if pp.i >= 0
                        && pp.i < N
                        && pp.j >= 0
                        && pp.j < M
                        && grid[pp.i as usize][pp.j as usize] != '.'
                    {
                        current.push(pp.clone());
                    }
                });
            }
        })
    });

    let mut visited: HashSet<Point> = HashSet::new();
    let mut d = 0;

    dfs(&g, &mut visited, &start, &mut d);

    println!("{}", (d + 1) / 2);
}
