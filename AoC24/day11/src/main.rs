use std::{collections::HashMap, usize};

fn blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (k, v) in stones {
        let num = format!("{}", k);
        match k {
            0 => *new_stones.entry(1).or_default() += v,
            _ => {
                if num.len() % 2 > 0 {
                    *new_stones.entry(2024 * k).or_default() += v;
                } else {
                    let l: u64 = num[..num.len() / 2].parse().unwrap();
                    let r: u64 = num[num.len() / 2..].parse().unwrap();
                    *new_stones.entry(l).or_default() += v;
                    *new_stones.entry(r).or_default() += v;
                }
            }
        };
    }
    new_stones
}

fn main() {
    let mut current: Vec<(u64, u64)> = vec![17639, 47, 3858, 0, 470624, 9467423, 5, 188]
        .iter()
        .map(|&v| (v as u64, 1))
        .collect();

    let mut stones: HashMap<u64, u64> = current.into_iter().collect();

    let blinks = 75;
    for _ in 0..blinks {
        stones = blink(stones);
    }
    let num_stones: u64 = stones.values().sum();
    println!("Number of stones after {} blinks: {}", blinks, num_stones);
}
