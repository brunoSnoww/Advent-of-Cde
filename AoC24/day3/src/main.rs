use regex::Regex;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let part1: u32 = re
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            x * y
        })
        .sum();
    println!("{:?}", part1);
}

fn extract_mul_pairs(input: &str) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();
    let mut mul_enabled = true; // Mul instructions are enabled by default
    let mut start = 0;

    while start < input.len() {
        // Check for `do()` or `don't()`
        if input[start..].starts_with("do()") {
            mul_enabled = true;
            start += 4; // Move past "do()"
        } else if input[start..].starts_with("don't()") {
            mul_enabled = false;
            start += 7; // Move past "don't()"
        }
        // Check for `mul(x,y)`
        else if input[start..].starts_with("mul(") {
            if mul_enabled {
                let mul_start = start + 4; // Skip "mul("
                let mul_end = match input[mul_start..].find(')') {
                    Some(end) => mul_start + end,
                    None => break, // No closing parenthesis
                };

                let args = &input[mul_start..mul_end];
                let parts: Vec<&str> = args.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                        pairs.push((x, y));
                    }
                }
            }
            start += 4; // Move past "mul("
        } else {
            start += 1; // Move to the next character
        }
    }
    pairs
}

fn part2(input: &str) {
    let pairs = extract_mul_pairs(input);
    let p = pairs.iter().map(|(x, y)| x * y).sum::<u32>();
    println!("{}", p);
}

fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}
