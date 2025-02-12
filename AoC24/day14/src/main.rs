use regex::Regex;
use std::io::{self, Write};
use std::{collections::HashMap, thread, time::Duration};

#[derive(Debug, Default, Clone)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
struct Robot {
    pub velocity: Position,
    pub current_pos: Position,
    pub N: i32,
    pub M: i32,
    initial_pos: Position,
}
// vx = dx/dt => xf = xo + vt =>
impl Robot {
    fn move_robot(&mut self, t: i32) {
        self.current_pos.x = self.initial_pos.x + t * self.velocity.x;
        self.current_pos.y = self.initial_pos.y + t * self.velocity.y;
        self.current_pos.x = self.current_pos.x.rem_euclid(self.M);
        self.current_pos.y = self.current_pos.y.rem_euclid(self.N);
    }
}

fn get_quadrant(p: Position, n: i32, m: i32) -> i32 {
    if p.x == m / 2 || p.y == n / 2 {
        return -1;
    }
    if p.x > m / 2 {
        return if p.y > n / 2 { 1 } else { 0 };
    }
    if p.y > n / 2 {
        2
    } else {
        3
    }
}

/// Returns `Some((p, v))` if successful, otherwise `None`.
fn extract_positions(input: &str) -> Option<(Position, Position)> {
    let re = Regex::new(r"p=(?P<px>-?\d+),(?P<py>-?\d+)\s+v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();

    re.captures(input).map(|captures| {
        let p = Position {
            x: captures["px"].parse().unwrap(),
            y: captures["py"].parse().unwrap(),
        };
        let v = Position {
            x: captures["vx"].parse().unwrap(),
            y: captures["vy"].parse().unwrap(),
        };
        (p, v)
    })
}

fn main() {
    let N = 103;
    let M = 101;
    let mut robots: Vec<Robot> = include_str!("./input.txt")
        .lines()
        .map(|l| extract_positions(l).unwrap())
        .map(|(p, v)| Robot {
            velocity: v,
            current_pos: p.clone(),
            N,
            M,
            initial_pos: p,
        })
        .collect();
    robots.iter_mut().for_each(|r| r.move_robot(100));
    let quadrants = robots
        .iter()
        .map(|r| get_quadrant(r.current_pos.clone(), N, M))
        .filter(|x| *x != -1)
        .fold(HashMap::new(), |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        });

    println!("{:?}", quadrants.values().fold(1, |x, y| x * y));
}
