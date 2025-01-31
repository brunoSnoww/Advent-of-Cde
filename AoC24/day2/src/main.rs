use std::cmp::PartialOrd;
use std::ops::Sub;

fn is_safe<T>(vec: Vec<T>) -> bool
where
    T: PartialOrd + Sub<Output = T> + From<u8> + Copy + std::fmt::Debug,
{
    if vec.len() < 2 {
        return true; // A vector with fewer than 2 elements is always "safe"
    }

    let n = vec.len() - 1;
    let is_increasing = vec[1] > vec[0];

    for i in 0..n {
        let diff = vec[i + 1] - vec[i];
        let abs_diff = if diff > T::from(0) {
            diff
        } else {
            T::from(0) - diff
        };

        // Check if the trend (increasing or decreasing) is violated
        if (is_increasing && diff < T::from(0))
            || (!is_increasing && diff > T::from(0) || diff == T::from(0))
            || abs_diff > T::from(3)
        {
            return false;
        }
    }

    true
}

fn make_vec(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

fn can_make_safe<T>(vec: Vec<T>) -> bool
where
    T: PartialOrd + Sub<Output = T> + From<u8> + Copy + std::fmt::Debug,
{
    if vec.len() <= 2 {
        return true; // A vector with 2 or fewer elements can always be made "safe"
    }

    for i in 0..vec.len() {
        let mut new_vec = vec.clone();
        new_vec.remove(i); // Remove the i-th element
        if is_safe(new_vec) {
            return true;
        }
    }

    false
}

fn main() {
    let part1: i32 = include_str!("./input.txt")
        .lines()
        .map(make_vec)
        .map(|v| can_make_safe(v) as i32)
        .sum();
    println!("{part1}");
}
