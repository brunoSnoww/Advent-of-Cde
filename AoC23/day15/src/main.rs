use indexmap::IndexMap;

fn hash(s: &str) -> i64 {
    let mut ans: i64 = 0;
    for c in s.chars() {
        ans += (c as u8 as i64);
        ans *= 17;
        ans %= 256;
    }
    ans
}

fn main() {
    let lines = include_str!("./test.txt").lines();
    let mut ans = 0;
    for l in lines.clone() {
        for s in l.split(',') {
            ans += hash(s);
        }
    }
    println!("Part1: {}", ans);
    // Part2
    let mut boxes: Vec<IndexMap<&str, u8>> = vec![IndexMap::new(); 256];
    for l in lines {
        for s in l.split(',') {
            if s.contains('=') {
                let (label, mirror) = s.split_once('=').unwrap();
                let mirror: u8 = mirror.parse().unwrap();
                let bx_idx = hash(label) as usize;
                boxes[bx_idx].insert(label, mirror);
            } else {
                let label = &s[0..s.len() - 1];
                boxes[hash(label) as usize].shift_remove(label);
            }
        }
    }
    let ans2 = boxes
        .iter()
        .enumerate()
        .map(|(bxIdx, map)| {
            map.values()
                .enumerate()
                .map(|(slot, v)| {
                    let vi32 = *v as i32;
                    (bxIdx as i32 + 1) * (slot as i32 + 1) * vi32
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Part2: {}", ans2);
}
