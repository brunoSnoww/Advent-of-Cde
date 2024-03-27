use std::{
    collections::{ HashSet},
};

fn part1() -> i64 {
    let mut hs = HashSet::new();
    let L = 25;
    let input: Vec<i64> = include_str!("./input.txt")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    for i in 0..L {
        hs.insert(input[i] as i64);
    }
    let mut p = 0;
    let mut ans = 0;
    input.iter().enumerate().skip(L).for_each(|(i, a)| {
        let mut found = false;
        for b in hs.clone() {
            let y = *a - b;
            if y != b && hs.contains(&y) {
                found = true;
                break;
            }
        }
        if !found {
            println!("Part1: {}", *a);
            ans = *a;
        }
        let x = input[p];
        hs.remove(&x);
        hs.insert(*a);
        p += 1;
    });
    ans
}

fn part2(x: i64) {
    let input: Vec<i64> = include_str!("./input.txt")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let n = input.len();
    for w in 2..n {
        for list in input.windows(w) {
            let sum = list.iter().sum::<i64>();
            if sum == x {
                println!(
                    "Part2: {}",
                    list.iter().min().unwrap() + list.iter().max().unwrap()
                );
            }
        }
    }
}

fn main() {
    part2(part1());
}
