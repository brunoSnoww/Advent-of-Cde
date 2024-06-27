use std::u32::MAX;

const PIXELS: [&str; 3] = ["  ", "██", "  "];
fn get_pixel(v: &Vec<i32>, row: usize, col: usize, width: usize) -> &str {
    for c in v.chunks(25 * 6) {
        let cc = c[row * width + col];
        if cc != 2 {
            return PIXELS[cc as usize];
        }
    }
    PIXELS[2]
}

fn main() {
    let input = include_str!("./input.txt")
        .lines()
        .fold(Vec::new(), |mut acc, l| {
            let p: Vec<i32> = l
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect();
            p.iter().for_each(|x| acc.push(*x));
            acc
        });
    let ans = MAX;
    let w1 = input
        .chunks(25 * 6)
        .min_by(|x, y| {
            let zx = x.iter().filter(|xx| **xx == 0).count();
            let zy = y.iter().filter(|yy| **yy == 0).count();
            zx.cmp(&zy)
        })
        .unwrap();
    let ones = w1.iter().filter(|x| **x == 1).count();
    let twos = w1.iter().filter(|x| **x == 2).count();

    let v = (0..6) /*for each row */
        .for_each(|row| {
            println!(
                "{:?}",
                (0..25) /* for each column */
                    .map(|col| get_pixel(&input, row, col, 25))
                    .collect::<String>()
            )
        });
    println!("{:?}", v);
    println!("{:?}", w1.len());
    println!("{:?}", ones * twos);
}
