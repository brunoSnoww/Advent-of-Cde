use std::collections::HashSet;

fn process(input: &str, range: i32) -> i32 {
    let mut i = 0;
    let mut j = range;
    for c in input.chars() {
        let mid = (i + j) / 2;
        if c == 'B' || c == 'R' {
            i = mid + 1;
        } else {
            j = mid;
        }
    }
    i.min(j)
}

fn id(input: &str) -> i32 {
    let rows = process(&input[0..7], 127);
    let cols = process(&input[7..], 7);
    rows * 8 + cols
}

fn main() {
    let input = include_str!("./input.txt").lines();
    let mut ans = i32::MIN;
    let mut hs = HashSet::new();
    for line in input {
        let s = id(line);
        if s != 1 && s != -1 {
            hs.insert(s);
        }
        ans = ans.max(s);
    }
    for i in (0..ans).rev() {
        if !hs.contains(&i) {
            println!("Part2: {}", i);
            break;
        }
    }
    println!("Part1: {}", ans);
}
