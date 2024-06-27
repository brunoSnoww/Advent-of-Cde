fn is_valid(n: i32) -> bool {
    let mut nn = n;
    let mut has_adj = false;
    let mut mx_so_far = nn % 10;
    let mut prev = -1;
    let mut cnt = 0;
    while nn > 0 {
        let r = nn % 10;
        if r > mx_so_far {
            return false;
        }
        if r == prev {
            cnt += 1;
        } else {
            if cnt == 2 {
                has_adj = true;
            }
            cnt = 1;
        }
        prev = r;
        mx_so_far = r;
        nn /= 10;
    }
    has_adj || cnt == 2
}

fn main() {
    let mut x = 0;
    for n in 130254..678276 {
        if is_valid(n) {
            x += 1;
        }
    }
    println!("{}", x);
}
