use std::collections::HashMap;

fn main() {
    let lines = include_str!("./input.txt").lines();
    let mut list1 = vec![];
    let mut list2 = vec![];

    let mut hs: HashMap<u32, u32> = HashMap::new();
    for line in lines {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|l| l.parse::<u32>().unwrap())
            .collect();
        let n1 = nums[0];
        let n2 = nums[1];
        list1.push(n1);
        list2.push(n2);
        *hs.entry(n2).or_insert(0) += 1;
    }
    list1.sort();
    list2.sort();
    let mut part1 = 0;
    let n = list1.len();
    for i in 0..n {
        part1 += (list1[i]).abs_diff(list2[i]);
    }
    println!("{}", part1);

    //Part2
    let part2: u32 = list1.iter().map(|n| (*n) * hs.get(n).unwrap_or(&0)).sum();
    println!("{:?}", part2);
}
