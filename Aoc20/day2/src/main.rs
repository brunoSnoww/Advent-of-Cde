use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<&str>> = include_str!("./input.txt")
        .lines()
        .map(|l| l.split(' ').collect())
        .collect();

    let total = input.len();
    let mut ans = 0;
    for vec in input.clone() {
        let n1: Vec<i32> = vec[0]
            .split("-")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let min = n1[0];
        let max = n1[1];
        //println!("{} {}", min, max);
        let mut hs = HashMap::new();
        let n2 = vec[1].chars().next().unwrap();
        let n3 = vec[2];
        n3.chars().for_each(|c| {
            *hs.entry(c).or_insert(0) += 1;
        });

        if let Some(times) = hs.get(&n2) {
            if (*times >= min) && (*times <= max) {
                ans += 1;
            }
        }
    }

    let mut part2 = 0;
    for vec in input {
        let n1: Vec<i32> = vec[0]
            .split("-")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let min = n1[0] as usize;
        let max = n1[1] as usize;
        //println!("{} {}", min, max);
        let n2 = vec[1].chars().next().unwrap();
        let n3 = vec[2];
        let a = n3.chars().nth(min - 1).unwrap();
        let b = n3.chars().nth(max - 1).unwrap();
        let mut p = 0;
        if a == n2 {
            p += 1;
        }

        if b == n2 {
            p += 1;
        }
        if p == 1 {
            part2 += 1;
        }
    }

    println!("{}", part2);
}
