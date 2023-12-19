use std::collections::{HashMap, HashSet};

struct State {
    pub id: String,
    pub rules: Vec<String>,
}
#[derive(Debug)]
struct Config {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl State {
    pub fn next_state(&self, idx: usize, config: &Config) -> Option<String> {
        let r = self.rules[idx].clone();
        if r.contains("<") || r.contains(">") {
            let sgn = if r.contains("<") { "<" } else { ">" };
            let parts: Vec<&str> = r.split(sgn).collect();
            let c = parts[0];
            let parts2: Vec<&str> = parts[1].split(":").collect();
            let val: i64 = parts2[0].parse().unwrap();
            let next_state = parts2[1];
            let config_val = match c {
                "x" => config.x,
                "m" => config.m,
                "a" => config.a,
                "s" => config.s,
                _ => unreachable!(),
            };
            let condition = if sgn == "<" {
                config_val < val
            } else {
                config_val > val
            };
            if condition {
                return Some(next_state.to_string());
            } else {
                return None;
            }
        }
        Some(r.to_string())
    }
    pub fn process(&self, config: &Config) -> String {
        for i in 0..self.rules.len() {
            if let Some(next_state) = self.next_state(i, config) {
                return next_state;
            }
        }
        unreachable!();
    }
}

fn main() {
    let g: Vec<&str> = include_str!("./input19.txt").split("\n\n").collect();
    let states_desc = g[0].lines();
    let mut states = HashMap::new();
    for s in states_desc {
        let parts: Vec<&str> = s.split("{").collect();
        let id = parts[0].to_string();
        let id2 = id.clone();
        let rules: Vec<String> = parts[1][..parts[1].len() - 1]
            .split(",")
            .map(|x| x.to_string())
            .collect();
        let st = State { id, rules };
        states.insert(id2, st);
    }

    let configs: Vec<Config> = g[1]
        .lines()
        .map(|config| {
            let parts: Vec<i64> = config[1..config.len() - 1]
                .split(",")
                .map(|s| {
                    let p: Vec<&str> = s.split("=").collect();
                    p[1].parse::<i64>().unwrap()
                })
                .collect();

            Config {
                x: parts[0],
                m: parts[1],
                a: parts[2],
                s: parts[3],
            }
        })
        .collect();

    let mut ans: i64 = 0;

    configs.iter().for_each(|c| {
        let mut curr_state = states.get(&"in".to_string()).unwrap();

        loop {
            let next_state = curr_state.process(c);
            if next_state == "A" || next_state == "R" {
                if next_state == "A" {
                    ans += c.x;
                    ans += c.m;
                    ans += c.a;
                    ans += c.s;
                }
                break;
            } else {
                curr_state = states.get(&next_state).unwrap();
            }
        }
    });

    println!("Part1: {}", ans);
}
