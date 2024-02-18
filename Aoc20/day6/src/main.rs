use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn main() {
    let groups: Vec<&str> = include_str!("./input.txt").split("\n\n").collect();
    let mut part2 = 0;
    groups.iter().for_each(|g| {
        let mut mp = HashMap::new();
        let mut len = 0;
        g.clone().lines().for_each(|l| {
            l.chars().for_each(|c| {
                *mp.entry(c).or_insert(0) += 1;
            });
            len += 1;
        });
        let l1 = g.lines().next().unwrap();
        l1.chars().for_each(|c| {
            if *mp.get(&c).unwrap() == len {
                part2 += 1;
            }
        });
    });
    println!("{}", part2);
}
