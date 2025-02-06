fn dfs(nums: &[u64], idx: usize, total: u64, curr: u64) -> Option<u64> {
    if idx + 1 == nums.len() {
        return if curr == total { Some(curr) } else { None };
    }
    if curr > total {
        return None;
    }

    let next_num = nums[idx + 1];

    // Try multiplication
    if let Some(res) = dfs(nums, idx + 1, total, curr * next_num) {
        return Some(res);
    }

    // Try addition
    if let Some(res) = dfs(nums, idx + 1, total, curr + next_num) {
        return Some(res);
    }

    // Try concatenation for part2
    let concat = format!("{}{}", curr, next_num).parse::<u64>().ok()?;
    if let Some(res) = dfs(nums, idx + 1, total, concat) {
        return Some(res);
    }

    None
}

fn check(input: &str) -> Option<u64> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let total = parts[0].parse::<u64>().ok()?;
    let nums: Vec<u64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?;

    if nums.is_empty() {
        return None;
    }

    dfs(&nums, 0, total, nums[0])
}

fn main() {
    let part2 = include_str!("./input.txt")
        .lines()
        .filter_map(check)
        .sum::<u64>();
    println!("{}", part2);
}
