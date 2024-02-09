use std::collections::VecDeque;

pub struct Input {
    edges: Vec<usize>,
    nodes: Vec<(usize, usize)>,
}

impl Input {
    #[inline]
    fn neighbours(&self, node: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (start, end) = self.nodes[node];
        (start..end).map(|edge| (edge, self.edges[edge]))
    }
}

pub fn parse(input: &str) -> Input {
    let mut lookup = vec![usize::MAX; 26 * 26 * 26];
    let mut neighbours = Vec::with_capacity(2_000);

    for line in input.lines().map(str::as_bytes) {
        let first = perfect_minimal_hash(&mut lookup, &mut neighbours, line);

        for chunk in line[5..].chunks(4) {
            let second = perfect_minimal_hash(&mut lookup, &mut neighbours, chunk);
            neighbours[first].push(second);
            neighbours[second].push(first);
        }
    }

    let mut edges = Vec::with_capacity(5_000);
    let mut nodes = Vec::with_capacity(neighbours.len());

    for list in neighbours {
        let start = edges.len();
        let end = edges.len() + list.len();
        edges.extend(list);
        nodes.push((start, end));
    }

    Input { edges, nodes }
}

pub fn part1(input: &Input) -> usize {
    // Arbitrarily pick the first node then find the furthest node from it.
    let start = furthest(input, 0);
    // Find the furthest node from start. The graph is constructed so that the minimum cut is
    // in the center of the graph, so start and end will be on opposite sides of the cut.
    let end = furthest(input, start);
    // Find the size of the graph still connected to start after the cut.
    let size = flow(input, start, end);
    size * (input.nodes.len() - size)
}

pub fn part2(_input: &Input) -> &'static str {
    "n/a"
}

fn perfect_minimal_hash(lookup: &mut [usize], nodes: &mut Vec<Vec<usize>>, slice: &[u8]) -> usize {
    // Base 26 index.
    let hash = slice[..3]
        .iter()
        .fold(0, |acc, b| 26 * acc + ((b - b'a') as usize));
    let mut index = lookup[hash];

    // First time seeing this key so push a new node and return its index.
    if index == usize::MAX {
        index = nodes.len();
        lookup[hash] = index;
        nodes.push(Vec::with_capacity(10));
    }

    index
}

/// BFS across the graph to find the furthest nodes from start.
fn furthest(input: &Input, start: usize) -> usize {
    let mut todo = VecDeque::new();
    todo.push_back(start);

    // The node indices are also their key so we can use a vec instead of a HashSet for speed.
    let mut seen = vec![false; input.nodes.len()];
    seen[start] = true;

    let mut result = start;

    while let Some(current) = todo.pop_front() {
        // The last node visited will be the furthest.
        result = current;

        for (_, next) in input.neighbours(current) {
            if !seen[next] {
                todo.push_back(next);
                seen[next] = true;
            }
        }
    }

    result
}

/// Simplified approach based on Edmonds–Karp algorithm.
fn flow(input: &Input, start: usize, end: usize) -> usize {
    let mut todo = VecDeque::new();
    // The path forms a linked list. During the BFS each path shares most nodes, so it's
    // more efficient both in space and speed to store the path as a linked list instead
    // of multiple copies of `vec`s.
    let mut path = Vec::new();
    // The capacity of each edge is 1 so only allow each edge to be used once.
    let mut used = vec![false; input.edges.len()];
    // The number of nodes from the 4th BFS is the size of one part of the cut graph.
    let mut result = 0;

    // We know the minimum cut is 3, so the 4th iteration will only be able to reach nodes
    // on start's side.
    for _ in 0..4 {
        todo.push_back((start, usize::MAX));
        result = 0;

        let mut seen = vec![false; input.nodes.len()];
        seen[start] = true;

        while let Some((current, head)) = todo.pop_front() {
            // Count how many nodes we visit.
            result += 1;

            // If we reached the end then add each edge of the path to `used`
            // so that it can be used only once.
            if current == end {
                let mut index = head;

                // Traverse the linked list.
                while index != usize::MAX {
                    let (edge, next) = path[index];
                    used[edge] = true;
                    index = next;
                }

                break;
            }

            // Find neighbouring nodes to explore, only allowing each edge to be used once.
            for (edge, next) in input.neighbours(current) {
                if !used[edge] && !seen[next] {
                    seen[next] = true;
                    todo.push_back((next, path.len()));
                    path.push((edge, head));
                }
            }
        }

        // Re-use for each iteration as a minor optimization.
        todo.clear();
        path.clear();
    }

    result
}

fn main() {
    let input = include_str!("./input25.txt");
    part1(input);
}
