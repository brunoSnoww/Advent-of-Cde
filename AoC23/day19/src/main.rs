use std::collections::HashMap;

fn dfs(
    states: &HashMap<String, Vec<(String, Vec<String>)>>,
    curr_state: String,
    cts: &HashMap<String, Vec<String>>,
    ans: &mut i64,
) {
    if curr_state == "A" {
        let mut x: i64 = 4000;
        let mut m: i64 = 4000;
        let mut a: i64 = 4000;
        let mut s: i64 = 4000;
        for c in ["x", "m", "a", "s"] {
            if let Some(ccs) = cts.get(c) {
                let mut min = 0;
                let mut max = 4001;
                for r in ccs {
                    if r.contains(">") {
                        let pts: Vec<&str> = r.split(">").collect();
                        min = min.max(pts[1].parse().unwrap());
                    }
                    if r.contains("<") {
                        let pts: Vec<&str> = r.split("<").collect();
                        max = max.min(pts[1].parse().unwrap());
                    }
                }
                let amount = max - min - 1;
                if c == "x" {
                    x = amount;
                } else if c == "m" {
                    m = amount;
                } else if c == "a" {
                    a = amount;
                } else {
                    s = amount;
                }
            }
        }
        *ans += (x * m * a * s);

        return;
    }

    if let Some(st) = states.get(&curr_state) {
        for (next, rules) in st {
            let mut cts2 = cts.clone();
            for r in rules {
                for c in ["x", "m", "a", "s"] {
                    if r.contains(c) {
                        cts2.entry(c.to_string())
                            .or_insert(Vec::new())
                            .push(r.clone());
                    }
                }
            }
            dfs(states, next.clone(), &cts2, ans);
        }
    }
}

fn inverse_rule(rule: &str) -> String {
    if rule.contains("<") {
        let parts: Vec<&str> = rule.split("<").collect();
        let ns = parts[0];
        let v: i64 = parts[1].parse().unwrap();
        let new_rule = format!("{}>{}", ns, v - 1);
        return new_rule;
    }
    if rule.contains(">") {
        let parts: Vec<&str> = rule.split(">").collect();
        let ns = parts[0];
        let v: i64 = parts[1].parse().unwrap();
        let new_rule = format!("{}<{}", ns, v + 1);
        return new_rule;
    }
    unreachable!();
}

fn main() {
    let g: Vec<&str> = include_str!("./input19.txt").split("\n\n").collect();
    let states_desc = g[0].lines();
    let mut states: HashMap<String, Vec<(String, Vec<String>)>> = HashMap::new();
    for s in states_desc {
        let parts: Vec<&str> = s.split("{").collect();
        let id = parts[0].to_string();
        let mut rules: Vec<(String, String)> = parts[1][..parts[1].len() - 1]
            .split(",")
            .map(|x| {
                if x.contains(":") {
                    let str = x.to_string();
                    let pts: Vec<&str> = str.split(":").collect();
                    return (pts[1].to_string(), pts[0].to_string());
                } else {
                    return (x.to_string(), "None".to_string());
                }
            })
            .collect();

        let mut acc: Vec<String> = Vec::new();
        for (next_state, rule) in rules {
            let mut acc2 = acc.clone();
            if rule != "None" {
                acc2.push(rule.to_string());
            }
            states
                .entry(id.clone())
                .or_insert(Vec::new())
                .push((next_state, acc2));

            if rule != "None" {
                acc.push(inverse_rule(&rule));
            }
        }
    }

    let mut ans: i64 = 0;
    let mut cts = HashMap::new();
    dfs(&states, "in".to_string(), &cts, &mut ans);
    println!("{}", ans);
}
