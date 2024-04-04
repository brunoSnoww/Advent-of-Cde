use num_complex::Complex;

fn parse_angle(angle: i64) -> Complex<i32> {
    match angle {
        90 | -270 => Complex { re: 0, im: 1 },
        180 | -180 => Complex { re: -1, im: 0 },
        270 | -90 => Complex { re: 0, im: -1 },
        _ => unreachable!(),
    }
}

fn part1() {
    let mut initial_v = Complex { re: 1, im: 0 };
    let mut initial_p = Complex { re: 0, im: 0 };
    let input = include_str!("./input.txt");
    input.lines().for_each(|l| {
        let v = l.chars().nth(0).unwrap();
        let n: i64 = l[1..].parse().unwrap();
        match v {
            'F' => {
                let dx = if initial_v.re == 0 {
                    0
                } else if initial_v.re > 0 {
                    n
                } else {
                    -n
                };
                let dy = if initial_v.im == 0 {
                    Complex { re: 0, im: 0 }
                } else if initial_v.im > 0 {
                    Complex { re: 0, im: n }
                } else {
                    Complex { re: 0, im: -n }
                };
                initial_p += dx;
                initial_p += dy;
            }
            'S' => {
                initial_p += Complex { re: 0, im: -n };
            }
            'N' => {
                initial_p += Complex { re: 0, im: n };
            }
            'E' => {
                initial_p += Complex { re: n, im: 0 };
            }
            'W' => {
                initial_p += Complex { re: -n, im: 0 };
            }
            'L' => {
                initial_v *= parse_angle(n);
            }
            'R' => {
                initial_v *= parse_angle(-n);
            }
            _ => unreachable!(),
        }
    });
    println!("Part1: {}", initial_p.re.abs() + initial_p.im.abs());
}

fn part2() {
    let mut initial_v = Complex { re: 1, im: 0 };
    let mut initial_p = Complex { re: 10, im: 1 };
    let input = include_str!("./input.txt");
    input.lines().for_each(|l| {
        let v = l.chars().nth(0).unwrap();
        let n: i64 = l[1..].parse().unwrap();
        match v {
            'F' => {
                let dx = if initial_v.re == 0 {
                    0
                } else if initial_v.re > 0 {
                    n
                } else {
                    -n
                };
                let dy = if initial_v.im == 0 {
                    Complex { re: 0, im: 0 }
                } else if initial_v.im > 0 {
                    Complex { re: 0, im: n }
                } else {
                    Complex { re: 0, im: -n }
                };
                initial_p += dx;
                initial_p += dy;
            }
            'S' => {
                initial_p += Complex { re: 0, im: -n };
            }
            'N' => {
                initial_p += Complex { re: 0, im: n };
            }
            'E' => {
                initial_p += Complex { re: n, im: 0 };
            }
            'W' => {
                initial_p += Complex { re: -n, im: 0 };
            }
            'L' => {
                initial_v *= parse_angle(n);
            }
            'R' => {
                initial_v *= parse_angle(-n);
            }
            _ => unreachable!(),
        }
    });
    println!("Part1: {}", initial_p.re.abs() + initial_p.im.abs());
}

fn main() {
    part1();
}
