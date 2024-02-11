use std::collections::HashSet;

fn byr(n: i32) -> bool {
    n >= 1920 && n <= 2002
}

fn iyr(n: i32) -> bool {
    n >= 2010 && n <= 2020
}

fn eyr(n: i32) -> bool {
    n >= 2020 && n <= 2030
}

fn height(h: &str) -> bool {
    let n = h.len();
    if n <= 2 {
        return false;
    }
    let ss = &h[(n - 2)..n];
    let num = (&h[0..(n - 2)]).parse::<i32>().unwrap();
    match ss {
        "cm" => num >= 150 && num <= 193,
        "in" => num >= 59 && num <= 76,
        _ => false,
    }
}

fn hcl(h: &str) -> bool {
    if h.len() != 7 {
        return false;
    }
    let mut i = h.chars();
    let c = i.next().unwrap();
    if c != '#' {
        return false;
    }
    i.all(|c| c.is_ascii_alphanumeric())
}

fn ecl(h: &str) -> bool {
    let v = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    v.iter().find(|s| *s == &h).is_some()
}

fn pid(s: &str) -> bool {
    s.len() == 9
}

fn main() {
    let blocks: Vec<&str> = include_str!("./input.txt").split("\n\n").collect();
    let mut ans = 0;
    let mut ans2 = 0;
    blocks.iter().for_each(|b| {
        let lines = b.lines();
        let mut hs: HashSet<_> = HashSet::new();
        let mut seen = 0;
        for l in lines {
            let mut els: Vec<Vec<&str>> = l
                .split_whitespace()
                .map(|el| el.split(":").collect::<Vec<&str>>())
                .collect();
            for e in els {
                hs.insert(e[0]);
                let key = e[0];
                let value = e[1];
                let is_valid = match key {
                    "pid" => pid(value),
                    "ecl" => ecl(value),
                    "hcl" => hcl(value),
                    "hgt" => height(value),
                    "eyr" => eyr(value.parse().unwrap()),
                    "iyr" => iyr(value.parse().unwrap()),
                    "byr" => byr(value.parse().unwrap()),
                    "cid" => true,
                    _ => unreachable!(),
                };
                if is_valid {
                    seen += 1;
                }
            }
        }
        if hs.len() == 8 || (hs.len() == 7 && !hs.contains("cid")) {
            ans += 1;
        }
        if !hs.contains("cid") {
            seen += 1;
        }
        if seen == 8 {
            ans2 += 1;
        }
    });

    println!("{:?} {}", ans, ans2);
}
