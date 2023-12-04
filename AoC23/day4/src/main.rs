use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input4.txt");
    let mut copies: HashMap<u32, i32> = HashMap::new();

    let part1: u32 = input
        .split('\n')
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            if line == "" {
                return 0;
            }
            *copies.entry(i as u32 + 1).or_insert(0) += 1;
            let mut iter = line.split('|');
            let mut hs: HashSet<i32> = HashSet::new();
            let game = iter.next().unwrap();
            let player = iter.next().unwrap_or("");
            game.split_whitespace().for_each(|num| {
                hs.insert(num.parse().unwrap());
            });
            let points: u32 = player.split_whitespace().fold(0, |acc, curr| {
                let num = curr.parse().unwrap_or(-1);
                if hs.contains(&num) {
                    return acc + 1;
                }
                return acc;
            });
            if points > 0 {
                for j in (i as u32) + 2..(i as u32) + points + 2 {
                    *copies.entry(j).or_insert(0) += *copies.entry(i as u32 + 1).or_insert(0);
                }
                return (2 as u32).pow(points - 1);
            }
            0
        })
        .sum();
    let part2: i32 = copies.values().sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
