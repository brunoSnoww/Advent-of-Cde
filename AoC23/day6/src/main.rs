fn main() {
    let mut input = include_str!("./test6.txt").split('\n');
    let times: Vec<u64> = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let dists: Vec<u64> = input
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let ans: Vec<u64> = times
        .iter()
        .enumerate()
        .map(|(i, curr)| {
            let mut ways = 0;
            for v in 1..(curr - 1) {
                let timeLeft = curr - v;
                let dist = dists[i as usize];
                let dtravelled = timeLeft * v;
                if dtravelled > dist {
                    ways += 1;
                }
            }
            ways
        })
        .collect();
    println!("{}", ans.iter().fold(1, |acc, c| acc * c));
}
