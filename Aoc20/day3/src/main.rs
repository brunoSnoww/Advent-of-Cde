fn solve(input: &Vec<Vec<char>>, di: usize, dj: usize) -> u64 {
    let n = input.len();
    let m = input[0].len();

    let mut pi = di;
    let mut pj = dj;
    let mut trees = 0;
    while pi < n {
        if input[pi][pj] == '#' {
            trees += 1;
        }
        pi += di;
        pj = (pj + dj) % m;
    }
    trees
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("./input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut p: u64 = 1;
    let slopes: Vec<Vec<usize>> = vec![vec![1, 1], vec![1, 3], vec![1, 5], vec![1, 7], vec![2, 1]];
    slopes.iter().for_each(|v| {
        p *= solve(&input, v[0], v[1]);
    });
    println!("{}", p);
}
