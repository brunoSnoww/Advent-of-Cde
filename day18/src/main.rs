use std::i64::MAX;

fn area(nodes: &Vec<(i64, i64)>, b: i64) -> i64 {
    let mut points = nodes.clone();

    // make sure it's closed
    // 2A = (x1 * y2 - y1 * x2) + (x2 * y3...)...

    let sum: i64 = points
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[0].1 * p[1].0 as i64)
        .sum::<i64>()
        .abs();

    // I don't know how to read math hieroglyphs
    // effed up somewhere? seems too complicated
    sum / 2 - b / 2 + 1 + b
}

fn main() {
    let input = include_str!("./input18.txt").lines();
    let mut steps: Vec<(&str, i64)> = Vec::new();
    input.for_each(|l| {
        let parts: Vec<&str> = l.split_whitespace().collect();
        let s = i64::from_str_radix(&parts[2][..parts[2].len() - 1], 16).unwrap();
        let dd = parts[2].chars().last().unwrap().to_digit(10).unwrap();

        let direct = if dd == 0 {
            "R"
        } else if dd == 1 {
            "D"
        } else if dd == 2 {
            "L"
        } else {
            "U"
        };
        steps.push((&direct, s));
    });
    let mut left = MAX;
    let mut right = -1;
    let mut top = MAX;
    let mut down = -1;

    let mut p = (0, 0);

    let mut vx = Vec::new();

    let mut b = 0;
    steps.iter().for_each(|(dir, s)| {
        let dx = if { *dir == "U" } {
            -1
        } else if { *dir == "D" } {
            1
        } else {
            0
        };
        let dy = if { *dir == "L" } {
            -1
        } else if { *dir == "R" } {
            1
        } else {
            0
        };

        for _ in 0..*s {
            p = (p.0 + dx, p.1 + dy);
            left = left.min(p.1);
            right = right.max(p.1);
            top = top.min(p.0);
            down = down.max(p.0);
        }

        vx.push(p);
        b += *s;
    });

    println!("{:?}", area(&vx, b));
    /*for i in top..down + 1 {
        for j in left..right + 1 {
            let pp = (i, j);
            if hs.contains(&pp) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    */
}
