use std::{collections::HashSet, i32::MAX};

type Point = (i32, i32);

fn mov(commmad: &str, x: &mut i32, y: &mut i32, vis: &mut HashSet<Point>) {
    let c = &commmad[0..1];
    let mut amount = commmad[1..].parse::<i32>().unwrap();
    let mut dx = 0;
    let mut dy = 0;
    match c {
        "R" => dx = 1,
        "L" => dx = -1,
        "U" => dy = 1,
        "D" => dy = -1,
        _ => unreachable!(),
    }
    while amount > 0 {
        *x += dx;
        *y += dy;
        vis.insert((*x, *y));
        amount -= 1;
    }
}

fn mov2(
    commmad: &str,
    x: &mut i32,
    y: &mut i32,
    vis: &mut HashSet<Point>,
    vis2: &mut HashSet<Point>,
) {
    let c = &commmad[0..1];
    let mut amount = commmad[1..].parse::<i32>().unwrap();
    let mut dx = 0;
    let mut dy = 0;
    match c {
        "R" => dx = 1,
        "L" => dx = -1,
        "U" => dy = 1,
        "D" => dy = -1,
        _ => unreachable!(),
    }
    while amount > 0 {
        if vis.contains(&(*x, *y)) {
            vis2.insert((*x, *y));
        }
        *x += dx;
        *y += dy;
        amount -= 1;
    }
}

fn steps(w1: &str, p: &Point) -> i32 {
    let mut s = 0;
    let mut x = 0;
    let mut y = 0;
    for w in w1.split(',') {
        let c = &w[0..1];
        let mut amount = w[1..].parse::<i32>().unwrap();
        let (dx, dy) = match c {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        while amount > 0 {
            x += dx;
            y += dy;
            s += 1;
            if x == p.0 && y == p.1 {
                return s;
            }
            amount -= 1;
        }
    }
    s
}

fn main() {
    let mut vis: HashSet<Point> = HashSet::new();
    let mut vis2: HashSet<Point> = HashSet::new();
    let mut wires = include_str!("./input.txt").lines();
    let mut wires2 = wires.clone();
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();
    let mut x = 0;
    let mut y = 0;
    wire1.lines().for_each(|w| {
        w.split(",").for_each(|ww| {
            mov(ww, &mut x, &mut y, &mut vis);
        })
    });
    x = 0;
    y = 0;
    wire2.lines().for_each(|w| {
        w.split(",").for_each(|ww| {
            mov2(ww, &mut x, &mut y, &mut vis, &mut vis2);
        })
    });

    let mut part1 = vis2
        .iter()
        .min_by(|a, b| {
            let a_dist = (a.0 + a.1).abs();
            let b_dist = (b.0 + b.1).abs();
            a_dist.cmp(&b_dist)
        })
        .unwrap();
    let w1 = wires2.next().unwrap();
    let w2 = wires2.next().unwrap();

    let mut part2 = MAX;
    vis2.iter().for_each(|p| {
        part2 = part2.min(steps(w1, p) + steps(w2, p));
    });

    println!("{:?}", part1.0.abs() + part1.1.abs());
    println!("{}", part2);
}
