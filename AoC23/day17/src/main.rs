use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

const UP: u32 = 1;
const DOWN: u32 = 2;
const LEFT: u32 = 3;
const RIGHT: u32 = 4;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    ans.insert(start, None);

    for (new, weight) in &graph[&start] {
        ans.insert(*new, Some((start, *weight)));
        prio.push(Reverse((*weight, *new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[&new] {
            Some((p, d)) if p == prev && d == dist_new => {}
            _ => continue,
        }
        for (next, weight) in &graph[&new] {
            match ans.get(next) {
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                Some(None) => {}
                _ => {
                    ans.insert(*next, Some((new, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, *next, new)));
                }
            }
        }
    }
    ans
}

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_default().insert(v2, c);
    graph.entry(v2).or_default();
}

fn main() {
    let mut g: Graph<u32, u32> = BTreeMap::new();
    let input: Vec<Vec<char>> = include_str!("./input17.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let n = input.len();
    let m = input[0].len();

    for i in 0..n {
        for j in 0..m {
            let idx = (i * m + j) as u32;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let ni = (i as i32) + dx;
                let nj = (j as i32) + dy;
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    let nidx = ni * (m as i32) + nj;
                    add_edge(&mut g, idx, nidx as u32, input[i][j].to_digit(10).unwrap());
                }
            }
        }
    }

    let d = dijkstra(&g, 0);
    println!("{:?}", d.get(&((n * m) as u32 - 1)).unwrap().unwrap().1);
}
