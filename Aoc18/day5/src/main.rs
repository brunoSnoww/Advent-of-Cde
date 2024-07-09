use std::{cmp::Ordering, collections::HashSet, i32::MAX};

fn reduce(input: &str) -> usize {
    let mut v = Vec::new();
    for c in input.chars() {
        match v.last() {
            None => v.push(c),
            Some(&d) => {
                if d.to_ascii_lowercase() == c.to_ascii_lowercase() && d != c {
                    v.pop();
                } else {
                    v.push(c);
                }
            }
        }
    }
    v.len()
}

fn main() {
    let mut input = include_str!("./input.txt");
    let mut p2 = MAX;
    for i in 0u8..=26 {
        let v = input
            .chars()
            .filter(|&c| c != (b'a' + i) as char && c != (b'A' + i) as char)
            .collect::<String>();
        p2 = p2.min(reduce(&v) as i32);
    }
    println!("{}", reduce(&input) - 1);
    println!("{}", p2 - 1);
}
