use std::io::{BufRead, Read};

fn proof_worf(input: String) -> i32 {
    let mut p = 10000;
    loop {
        let input = format!("{}{}", input, p.to_string());
        let hash = &md5::compute(input.as_bytes());
        let hx = format!("{:x}", hash);
        if hx.starts_with("000000") {
            break;
        }
        p += 1;
    }
    p
}

fn main() {
    let p = proof_worf("iwrupvqb".into());
    println!("{p}");
}
