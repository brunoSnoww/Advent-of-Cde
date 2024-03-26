use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut instructions: Vec<(&str, i32)> = Vec::new();
    let mut jmps = Vec::new();
    input.lines().enumerate().for_each(|(ii, l)| {
        let tmp: Vec<&str> = l.split_whitespace().collect();
        let i = tmp[0];
        if i == "nop" {
            instructions.push((i, 0));
        }
        if i == "acc" || i == "jmp" {
            if i == "jmp" {
                jmps.push(ii);
            }
            instructions.push((i, tmp[1].parse::<i32>().unwrap()));
        }
    });
    for j in jmps {
        let mut found = true;
        let mut seen: HashSet<i32> = HashSet::new();
        let mut acc = 0;
        let mut curr: i32 = 0;
        loop {
            if curr >= instructions.len() as i32 {
                break;
            }
            if seen.contains(&curr) {
                found = false;
                break;
            }
            seen.insert(curr);
            let a = instructions[curr as usize];
            if a.0 == "nop" {
                curr += 1;
            } else if a.0 == "jmp" {
                if curr == j as i32 {
                    curr += 1;
                } else {
                    curr += a.1;
                }
            } else {
                acc += a.1;
                curr += 1;
            }
        }
        if found {
            println!("{}", acc);
        }
    }
}
