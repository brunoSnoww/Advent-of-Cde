use std::collections::HashMap;

fn main() {
    let limits = HashMap::from([("blue", 14u32), ("red", 12u32), ("green", 13u32)]);
    let bytes = include_str!("./input2.txt");
    let part1: i32 = bytes
        .lines()
        .enumerate()
        .map(|(i, line)| {
            for set in line.split(';') {
                let color_pairs: Vec<&str> = set.split(',').collect();
                for pair in color_pairs {
                    let parts: Vec<&str> = pair.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        let color = parts[1];
                        let count = parts[0].parse::<u32>().unwrap_or(0);
                        let limit = *limits.get(color).unwrap();
                        if count > limit {
                            return 0;
                        }
                    }
                }
            }
            (i + 1) as i32
        })
        .sum();

    let part2: i32 = bytes
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut x1: i32 = 0;
            let mut x2: i32 = 0;
            let mut x3: i32 = 0;
            for set in line.split(';') {
                let color_pairs: Vec<&str> = set.split(',').collect();
                for pair in color_pairs {
                    let parts: Vec<&str> = pair.trim().split_whitespace().collect();
                    if parts.len() == 2 {
                        let color = parts[1];
                        let count = parts[0].parse::<i32>().unwrap_or(0);
                        if color == "blue" {
                            x1 = x1.max(count);
                        }
                        if color == "red" {
                            x2 = x2.max(count);
                        }
                        if color == "green" {
                            x3 = x3.max(count);
                        }
                    }
                }
            }
            (x1 * x2 * x3)
        })
        .sum();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
