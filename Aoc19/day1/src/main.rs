fn fuel_req(mass: i128) -> i128 {
    let mut p: i128 = (mass / 3);
    p - 2
}

fn fuel_req2(mut mass: i128) -> i128 {
    let mut s = -mass;
    let mut m = mass;
    while m > 0 {
        s += m;
        m = fuel_req(m);
    }
    s
}

fn main() {
    let masses: Vec<i128> = include_str!("./input.txt")
        .lines()
        .map(|l| l.parse::<i128>().unwrap())
        .collect();
    let part1 = masses.iter().map(|m| fuel_req(*m)).sum::<i128>();
    let part2 = masses.iter().map(|m| fuel_req2(*m)).sum::<i128>();
    println!("{}", part1);
    println!("{}", part2);
}
