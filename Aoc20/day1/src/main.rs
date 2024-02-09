use std::collections::HashSet;

fn main() {
    //part1
    let input: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut hs: HashSet<i32> = HashSet::new();
    let T = 2020;
    input.iter().for_each(|l| {
        let n = l.parse::<i32>().unwrap();
        if hs.contains(&(T - n)) {
            println!("{} {}", n, T - n);
            println!("{}", n * (T - n));
        }
        hs.insert(n);
    });

    //part2
    let numbers: Vec<i32> = include_str!("./input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    let n = numbers.len();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if numbers[i] + numbers[j] + numbers[k] == T {
                    println!(
                        "{} {} {} {}",
                        numbers[i],
                        numbers[j],
                        numbers[k],
                        numbers[i] * numbers[j] * numbers[k]
                    );
                }
            }
        }
    }
}

// x + y + z = 2020
