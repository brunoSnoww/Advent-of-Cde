use std::collections::{HashMap, HashSet};

fn ways(pattern: &str, count: &Vec<i64>, cache: &mut HashMap<(String, Vec<i64>), i64>) -> i64 {
    //first, base cases
    if pattern.len() == 0 {
        if count.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if count.len() == 0 {
        if pattern.contains("#") {
            return 0;
        }
        return 1;
    }

    let mut result = 0;
    let key = (pattern.clone().to_string(), count.clone());
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    // count cases
    let first_char = pattern.chars().nth(0).unwrap();
    if "?.".contains(first_char) {
        result += ways(&pattern[1..], count, cache);
    }

    let c = count[0] as usize;
    if "#?".contains(first_char) {
        if c <= pattern.len() {
            let block = &pattern[..c];
            if !block.contains(".")
                && (pattern.len() == c || pattern.chars().nth(c).unwrap() != '#')
            {
                result += ways(
                    &pattern[(c + 1).min(pattern.len())..],
                    &count.clone()[1..].to_vec(),
                    cache,
                );
            }
        }
    }
    cache.insert(key, result);
    result
}

fn main() {
    let input: Vec<&str> = include_str!("./input12.txt").lines().collect();
    let mut ans: i64 = 0;
    let mut cache: HashMap<(String, Vec<i64>), i64> = HashMap::new();
    for l in input {
        let P: Vec<&str> = l.split_whitespace().collect();
        let repeated_str: String = std::iter::repeat(P[0])
            .take(5)
            .collect::<Vec<&str>>()
            .join("?");
        let count: Vec<i64> = P[1].split(',').map(|x| x.parse().unwrap()).collect();
        ans += ways(&repeated_str, &count.repeat(5), &mut cache);
    }
    println!("{}", ans);
}
