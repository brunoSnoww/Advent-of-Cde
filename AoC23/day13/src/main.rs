fn check_mirror_row(grid: &str, ans: &mut i32, is_row: bool) -> bool {
    let lines: Vec<&str> = grid.lines().collect();
    let n = lines.len();
    for mirror_begin in (1..n) {
        for mirror_end in (mirror_begin + 1..n) {
            let mirror = &lines[mirror_begin..mirror_end];
            let all_equal = mirror.iter().all(|x| *x == mirror[0]);
            if !all_equal {
                continue;
            }

            let mut block1: String = lines[0..mirror_begin].join("");
            let mut block2: String = lines[mirror_end + 1..]
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect();

            if block1.len() > block2.len() {
                let tmp = block2.clone();
                block2 = block1;
                block1 = tmp;
            }
            if block2.ends_with(&block1) || block1.is_empty() {
                *ans += if is_row {
                    100 * (mirror_begin + 1) as i32
                } else {
                    1 + mirror_begin as i32
                };
                return true;
            }
        }
    }
    false
}

fn check_mirror_column(grid: &str, ans: &mut i32) -> bool {
    let lines: Vec<&str> = grid.lines().collect();
    let transposed_lines: Vec<String> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect()
        })
        .collect();
    let transposed_grid: String = transposed_lines.join("\n");

    check_mirror_row(&transposed_grid, ans, false)
}

fn main() {
    let input = include_str!("./input13.txt");
    let mut ans = 0;
    let grids = input.split("\n\n");
    grids.for_each(|x| {
        let c = check_mirror_column(x, &mut ans);
        if !c {
            check_mirror_row(x, &mut ans, true);
        }
    });

    println!("{}", ans);
}
