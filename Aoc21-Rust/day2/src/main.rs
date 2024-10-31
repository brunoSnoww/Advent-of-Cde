fn part1() {
    let input = include_str!("./input.txt");
    let mut d = 0;
    let mut h = 0;
    for line in input.lines() {
        let mut l = line.split_whitespace();
        let c = l.next().unwrap();
        let v = l.next().unwrap().parse::<i32>().unwrap();
        if c == "forward" {
            h += v;
        } else {
            d += if c == "down" { v } else { -v };
        }
    }
    println!("{} ", d * h);
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut aim = 0;
    let mut h = 0;
    let mut d = 0;
    for line in input.lines() {
        let mut l = line.split_whitespace();
        let c = l.next().unwrap();
        let v = l.next().unwrap().parse::<i32>().unwrap();
        if c == "forward" {
            h += v;
            d += aim * v;
        } else {
            aim += if c == "down" { v } else { -v };
        }
    }
    println!("{} ", d * h);
}

fn main() {
    part1();
    part2();
}
