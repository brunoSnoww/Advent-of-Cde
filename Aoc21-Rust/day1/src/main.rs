use std::thread::spawn;

fn part1() {
    let input = include_str!("./input.txt");
    let mut s = 0;
    let mut it = input.lines();
    let mut n = it.next().unwrap().parse::<u32>().unwrap();
    for line in it {
        let m = line.parse::<u32>().unwrap();
        s += if m > n { 1 } else { 0 };
        n = m;
    }
    println!("{}", s);
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut it = input.lines();
    let mut n1 = it.next().unwrap().parse::<u32>().unwrap();
    let mut n2 = it.next().unwrap().parse::<u32>().unwrap();
    let mut n3 = it.next().unwrap().parse::<u32>().unwrap();
    let mut s = n1 + n2 + n3;
    let mut ans = 0;
    for line in it {
        let n4 = line.parse::<u32>().unwrap();
        let s2 = s + n4 - n1;
        if s2 > s {
            ans += 1;
        }
        s = s2;
        n1 = n2;
        n2 = n3;
        n3 = n4;
    }
    println!("{}", ans);
}

fn main() {
    let t1 = spawn(move || {
        part1();
    });
    let t2 = spawn(move || {
        part2();
    });

    let _ = t1.join();
    let _ = t2.join();
}
