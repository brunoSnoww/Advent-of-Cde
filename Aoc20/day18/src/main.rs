use std::{collections::VecDeque, thread};

fn precedence(op: char, part: bool) -> u8 {
    match op {
        '+' | '-' => {
            if !part {
                1
            } else {
                2
            }
        }
        '*' | '/' => 1,
        _ => 0,
    }
}

fn reduce_pilha(values: &mut VecDeque<u64>, ops: &mut VecDeque<char>) {
    let val2 = values.pop_back().unwrap();
    let val1 = values.pop_back().unwrap();
    let o = ops.pop_back().unwrap();
    let ula_result = op(o, val1, val2);
    values.push_back(ula_result);
}

fn op(op: char, a: u64, b: u64) -> u64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => unreachable!(),
    }
}

fn evaluate(tokens: &str, part: bool) -> u64 {
    let mut ops = VecDeque::new();
    let mut values: VecDeque<u64> = VecDeque::new();
    let mut i = 0;
    let n = tokens.len();
    while i < n {
        let c = tokens.chars().nth(i).unwrap();
        match c {
            '(' => ops.push_back('('),
            '0'..='9' => {
                let mut v: u64 = 0;
                while i < n && tokens.chars().nth(i).unwrap().is_ascii_digit() {
                    let tmp_char = tokens.chars().nth(i).unwrap();
                    let d = tmp_char.to_digit(10).unwrap() as u64;
                    v = 10 * v + d;
                    i += 1;
                }
                i -= 1;
                values.push_back(v);
            }
            ')' => {
                while !ops.is_empty() && *ops.back().unwrap() != '(' {
                    reduce_pilha(&mut values, &mut ops);
                }
                if !ops.is_empty() {
                    ops.pop_back().unwrap();
                }
            }
            '+' | '*' => {
                while !ops.is_empty()
                    && precedence(*ops.back().unwrap(), part) >= precedence(c, part)
                {
                    reduce_pilha(&mut values, &mut ops);
                }
                ops.push_back(c);
            }
            _ => {}
        }
        i += 1;
    }
    while !ops.is_empty() {
        reduce_pilha(&mut values, &mut ops);
    }
    let ret = *values.back().unwrap();
    ret
}

fn main() {
    let t1 = thread::spawn(move || {
        let input = include_str!("./input.txt");
        let mut ans = 0;
        for line in input.lines() {
            ans += evaluate(line, false);
        }
        ans
    });
    let t2 = thread::spawn(move || {
        let input = include_str!("./input.txt");
        let mut ans = 0;
        for line in input.lines() {
            ans += evaluate(line, true);
        }
        ans
    });
    let res1 = t1.join().unwrap();
    let res2 = t2.join().unwrap();
    println!("Part1: {}", res1);
    println!("Part2: {}", res2);
}
