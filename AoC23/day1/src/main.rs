const replacements: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn main() {
    println!(
        "{}",
        include_bytes!("./input1.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                if let (Some(d1), Some(d2)) = (
                    (0..line.len()).find_map(|i| number(line, i)),
                    (0..line.len()).rev().find_map(|i| number(line, i)),
                ) {
                    10 * d1 + d2
                } else {
                    return 0;
                }
            })
            .sum::<usize>()
    );
}

fn number(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(replacements
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}
