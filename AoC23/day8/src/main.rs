use std::collections::HashMap;

fn main() {
    let input = include_str!("./input8.txt");
    let mut g: HashMap<&str, (&str, &str)> = HashMap::new();

    let steps = input.lines().take(1).next().unwrap();

    let mut positionsA: Vec<&str> = Vec::new();
    input.lines().skip(2).for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        g.insert(parts[0], (parts[1], parts[2]));
        if parts[0].ends_with("A") {
            positionsA.push(parts[0]);
        }
    });

    /* PART 1
    let mut c = "AAA";
    let mut i: usize = 0;
    let mut s = 1;
    loop {
        let d = g.get(c).unwrap();
        let dir = steps.chars().nth(i).unwrap();

        i = (i + 1) % steps.len() as usize;

        match dir {
            'L' => c = d.0,
            'R' => c = d.1,
            _ => unreachable!(),
        }
        if c == "ZZZ" {
            break;
        }
        s += 1;
    }
    println!("Part 1: {}", s);*/

    //THIS ONLY WORKS BECAUSE THE INPUT IS BUILT IN SUCH A WAY THAT
    //THE AMOUNT OF TIME IT TAKES FOR A GHOST TO FROM A TO Z IS THE BACKWARDS
    //SO LCM WORKS
    let mut cycles: Vec<i64> = Vec::new();
    let mut i = 0;
    for p in positionsA {
        let mut s = 1;
        let mut _p = p;
        loop {
            let d = g.get(_p).unwrap();
            let dir = steps.chars().nth(i).unwrap();

            i = (i + 1) % steps.len();

            match dir {
                'L' => _p = d.0,
                'R' => _p = d.1,
                _ => unreachable!(),
            }
            if _p.ends_with('Z') {
                break;
            }
            s += 1;
        }
        cycles.push(s);
    }
    println!(
        "{:?}",
        cycles
            .iter()
            .fold(1, |acc, curr| num::integer::lcm(acc, *curr))
    );

    //Part2
}
