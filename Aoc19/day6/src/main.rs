use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut hs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut inDegree: HashSet<&str> = HashSet::new();
    include_str!("./input.txt").lines().for_each(|l| {
        let vec: Vec<&str> = l.split(")").collect();
        let u = vec[0];
        let v = vec[1];
        hs.entry(&v).or_insert_with(|| Vec::new()).push(u);
        hs.entry(&u).or_insert_with(|| Vec::new()).push(v);
        inDegree.insert(v);
    });
    let mut d = HashMap::new();
    //let src = hs.iter().find(|(k, _)| !inDegree.contains(*k)).unwrap().0;
    let src = "YOU";
    d.insert(src, 0);
    let mut q = VecDeque::new();
    q.push_back(src);
    while q.len() > 0 {
        let mut u = q.pop_front().unwrap();
        let curr_dist = *d.get(u).unwrap();
        match hs.get(u) {
            Some(list) => {
                for v in list {
                    if !d.contains_key(v) {
                        d.insert(v, curr_dist + 1);
                        q.push_back(v);
                    }
                }
            }
            None => {}
        }
    }

    let part1: i32 = d.values().map(|&x| (1 + x) * x / 2).sum();
    println!("{}", part1);

    let part2 = *d.get(&"SAN").unwrap();
    println!("{}", part2 - 2);
}
