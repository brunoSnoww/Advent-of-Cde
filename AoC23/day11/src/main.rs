use std::collections::HashSet;

fn part(input: &str, cost: usize) -> usize {
    let grid: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();
    let xs = grid.iter().map(|&(x, _)| x).collect::<HashSet<_>>();
    let ys = grid.iter().map(|&(_, y)| y).collect::<HashSet<_>>();
    let items = grid.iter().collect::<Vec<_>>();
    let mut total = 0;
    for (i, &&(sx, sy)) in items.iter().enumerate() {
        for &&(dx, dy) in items.iter().skip(i + 1) {
            for x in sx.min(dx)..sx.max(dx) {
                total += if xs.contains(&x) { 1 } else { cost };
            }
            for y in sy.min(dy)..sy.max(dy) {
                total += if ys.contains(&y) { 1 } else { cost };
            }
        }
    }
    total
}

fn main() {
    let s = include_str!("./input11.txt");
    println!("Part1: {}", part(s, 2));
    println!("Part2: {}", part(s, 1000000));
}
