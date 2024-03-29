fn getNbrs(i: i32, j: i32, N: i32, M: i32, g: &Vec<Vec<char>>) -> i32 {
    let mut p = 0;
    let mut ii = i;
    let mut jj = j;
    while ii + 1 < N {
        ii += 1;
        if g[ii as usize][j as usize] == '#' {
            p += 1;
            break;
        }

        if g[ii as usize][j as usize] == 'L' {
            break;
        }
    }
    jj = j;
    while jj + 1 < M {
        jj += 1;
        if g[i as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[i as usize][jj as usize] == 'L' {
            break;
        }
    }
    ii = i;
    while ii >= 1 {
        ii -= 1;
        if g[ii as usize][j as usize] == '#' {
            p += 1;
            break;
        }
        if g[ii as usize][j as usize] == 'L' {
            break;
        }
    }
    jj = j;
    while jj >= 1 {
        jj -= 1;
        if g[i as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[i as usize][jj as usize] == 'L' {
            break;
        }
    }
    ii = i;
    jj = j;
    while ii >= 1 && jj >= 1 {
        jj -= 1;
        ii -= 1;
        if g[ii as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[ii as usize][jj as usize] == 'L' {
            break;
        }
    }
    ii = i;
    jj = j;
    while ii + 1 < N && jj >= 1 {
        jj -= 1;
        ii += 1;
        if g[ii as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[ii as usize][jj as usize] == 'L' {
            break;
        }
    }
    ii = i;
    jj = j;
    while ii + 1 < N && jj + 1 < M {
        jj += 1;
        ii += 1;
        if g[ii as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[ii as usize][jj as usize] == 'L' {
            break;
        }
    }
    ii = i;
    jj = j;
    while ii >= 1 && jj + 1 < M {
        jj += 1;
        ii -= 1;
        if g[ii as usize][jj as usize] == '#' {
            p += 1;
            break;
        }
        if g[ii as usize][jj as usize] == 'L' {
            break;
        }
    }
    p
}

fn areEqual(g1: &Vec<Vec<char>>, g2: &Vec<Vec<char>>) -> bool {
    for (i, v1) in g1.iter().enumerate() {
        let v2 = g2[i].clone();
        if !v1.eq(&v2) {
            return false;
        }
    }
    true
}

fn main() {
    let mut graph: Vec<Vec<char>> = include_str!("./input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let N = graph.len() as i32;
    let M = graph[0].len() as i32;
    loop {
        let mut next_graph: Vec<Vec<char>> = vec![];
        graph.iter().enumerate().for_each(|(i, v)| {
            let mut next_row: Vec<char> = vec![];
            v.iter().enumerate().for_each(|(j, c)| {
                let curr = *c;
                let nbs = getNbrs(i as i32, j as i32, N, M, &graph);
                match curr {
                    'L' => {
                        if nbs == 0 {
                            next_row.push('#');
                        } else {
                            next_row.push('L')
                        }
                    }
                    '#' => {
                        if nbs >= 5 {
                            next_row.push('L');
                        } else {
                            next_row.push('#');
                        }
                    }
                    '.' => {
                        next_row.push('.');
                    }
                    _ => unreachable!(),
                }
            });

            next_graph.push(next_row.clone());
        });
        if areEqual(&graph, &next_graph) {
            break;
        }
        graph = next_graph;
    }
    let part1 = graph.iter().flatten().filter(|c| **c == '#').count();
    println!("{:?}", part1);
}
