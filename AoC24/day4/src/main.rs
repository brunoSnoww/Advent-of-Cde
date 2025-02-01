static D: &[(i32, i32)] = &[
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(input: &str) {
    let grid = make_grid(input);
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;

    (0..n).for_each(|i| {
        for j in 0..m {
            if grid[i][j] == 'X' {
                for &(di, dj) in D {
                    let mut ni = i as i32;
                    let mut nj = j as i32;
                    let mut s = "X".to_string();
                    for _ in 0..3 {
                        ni += di;
                        nj += dj;
                        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                            s.push(grid[ni as usize][nj as usize]);
                        } else {
                            break;
                        }
                    }
                    if s == "XMAS" {
                        ans += 1;
                    }
                }
            }
        }
    });
    println!("{}", ans);
}

fn sort_string(input: &str) -> String {
    // Collect the string into a vector of chars
    let mut chars: Vec<char> = input.chars().collect();

    // Sort the vector in ascending Unicode order
    chars.sort();

    // Re-create a String from the sorted chars
    chars.into_iter().collect()
}

fn part2(input: &str) {
    let grid = make_grid(input);
    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;

    (0..n).for_each(|i| {
        for j in 0..m {
            if grid[i][j] == 'A' && i + 1 < n && j + 1 < m && i > 0 && j > 0 {
                let s1 = sort_string(&format!("{}A{}", grid[i - 1][j - 1], grid[i + 1][j + 1]));
                let s2 = sort_string(&format!("{}A{}", grid[i + 1][j - 1], grid[i - 1][j + 1]));

                if s1 == "AMS" && s1 == s2 {
                    ans += 1;
                }
            }
        }
    });
    println!("{ans}");
}

fn main() {
    let input = include_str!("./test.txt");
    part1(input);
    part2(input);
}
