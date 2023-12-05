struct P {
    dest: u64,
    source: u64,
    range: u64,
}
fn main() {
    let mut iter = include_str!("./input5.txt").split("\n");
    let mut seeds: Vec<u64> = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let groups = include_str!("./input5.txt").split("\n\n");

    groups.for_each(|g| {
        let map: Vec<P> = g
            .split("\n")
            .skip(1)
            .filter_map(|l| {
                let parts: Vec<u64> = l
                    .split_whitespace()
                    .map(|x| x.parse::<u64>())
                    .collect::<Result<Vec<u64>, _>>()
                    .ok()?;

                if parts.len() >= 3 {
                    let dest = parts[0];
                    let source = parts[1];
                    let range = parts[2];
                    Some(P {
                        dest,
                        range,
                        source,
                    })
                } else {
                    None
                }
            })
            .collect();

        seeds = seeds
            .iter()
            .map(|&x| {
                let r = map.iter().find(|y| x >= y.source && x < y.source + y.range);
                match r {
                    Some(val) => return x - val.source + val.dest,
                    None => return x,
                }
            })
            .collect();
    });

    println!("part 1:{:?}", seeds.iter().min().unwrap());
}
// 52 50 98
// 79 - 50 + 52 -> 81
