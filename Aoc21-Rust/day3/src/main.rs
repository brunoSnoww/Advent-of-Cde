use std::collections::HashMap;

fn part1() {
    let input = include_str!("./input.txt");
    let mut ones: HashMap<usize, u32> = HashMap::new();
    let n: usize = 12; // Number of bits per line

    // Count the occurrences of '1' at each bit position
    for line in input.lines() {
        for (b, v) in line.chars().enumerate() {
            let c = v.to_digit(10).unwrap();
            *ones.entry(b).or_insert(0) += c;
        }
    }

    // Total number of lines
    let line_count = input.lines().count() as u32;

    // Calculate the majority bit for each position
    let mut ans = HashMap::new();
    for i in 0..n {
        let count_of_ones = *ones.get(&i).unwrap_or(&0);
        let count_of_zeros = line_count - count_of_ones;
        let bit = if count_of_ones > count_of_zeros { 1 } else { 0 };
        ans.insert(i, bit);
    }

    // Build the final binary string
    let mut a = String::new();
    for i in 0..n {
        let bit = ans.get(&i).unwrap();
        a.push_str(&bit.to_string());
    }
    let num = u32::from_str_radix(&a, 2).unwrap();
    let len = a.len();
    let f = !num & ((1 << len) - 1);

    // Output the final binary string
    println!("{:?}", num * f);
}

fn get_nums(nums: &Vec<String>, pos: usize, b: bool) -> Vec<String> {
    let mut ones = 0;
    let len = nums.len() as u32;
    let mut ans = Vec::new();
    for n in nums {
        let c = n.chars().nth(pos).unwrap();
        ones += c.to_digit(10).unwrap();
    }
    let zeros = len - ones;
    // Determine which character to add based on the count and consider_zero flag
    let to_add = if (ones >= zeros) == b { '1' } else { '0' };
    for n in nums {
        let c = n.chars().nth(pos).unwrap();
        if c == to_add {
            ans.push(n.to_string());
        }
    }
    ans
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut nums: Vec<String> = input.lines().map(String::from).collect();
    let mut nums2 = nums.clone();
    let p = nums[0].len();
    for i in 0..p {
        if nums.len() > 1 {
            nums = get_nums(&nums, i, true);
        }

        if nums2.len() > 1 {
            nums2 = get_nums(&nums2, i, false);
        }
    }

    let n1 = u32::from_str_radix(&nums[0], 2).unwrap();
    let n2 = u32::from_str_radix(&nums2[0], 2).unwrap();

    println!("{}", n1 * n2);
}

fn main() {
    part2();
}
