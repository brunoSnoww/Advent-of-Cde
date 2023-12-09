fn p1(v: &Vec<i64>) -> i64 {
    let diff: Vec<i64> = v.windows(2).map(|a| a[1] - a[0]).collect();
    let allZero = diff.iter().all(|x| *x == 0);
    if allZero {
        return *v.last().unwrap();
    }
    return v.last().unwrap() + p1(&diff);
}

fn p2(v: &Vec<i64>) -> i64 {
    let diff: Vec<i64> = v.windows(2).map(|a| a[1] - a[0]).collect();
    let allZero = diff.iter().all(|x| *x == 0);
    if allZero {
        return *v.first().unwrap();
    }
    return v.first().unwrap() - p2(&diff);
}

fn main() {
    let (ans1, ans2) = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap() as i64)
                .collect()
        })
        .map(|x: Vec<i64>| (p1(&x), p2(&x)))
        .fold((0, 0), |acc, tuple| (acc.0 + tuple.0, acc.1 + tuple.1));

    println!("Part1 : {}", ans1);
    println!("Part2 : {}", ans2);
}
