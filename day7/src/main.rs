use std::collections::HashMap;

fn score_card(c: char) -> i64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '2'..='9' => c.to_digit(10).unwrap() as i64,
        _ => unreachable!(),
    }
}

fn rank(cards0: &str, cards1: &str) -> std::cmp::Ordering {
    let c0_score = get_type(cards0);
    let c1_score = get_type(cards1);

    if c0_score < c1_score {
        return std::cmp::Ordering::Less;
    } else if c0_score > c1_score {
        return std::cmp::Ordering::Greater;
    }

    for (c0, c1) in std::iter::zip(cards0.chars(), cards1.chars()) {
        if score_card(c0) < score_card(c1) {
            return std::cmp::Ordering::Less;
        } else if score_card(c0) > score_card(c1) {
            return std::cmp::Ordering::Greater;
        }
    }

    std::cmp::Ordering::Equal
}
type Card = (String, i64);

const FIVE: i64 = 7;
const FOUR: i64 = 6;
const FULLHOUSE: i64 = 5;
const THREE: i64 = 4;
const TWO: i64 = 3;
const ONE: i64 = 2;
const HIGH: i64 = 1;

fn get_type(card: &str) -> i64 {
    let mut mp: HashMap<char, i64> = HashMap::new();
    for c in card.chars() {
        *mp.entry(c).or_insert(0) += 1;
    }
    let mut values: Vec<_> = mp.values().clone().collect();
    values.sort();

    let j_count = card.chars().filter(|&c| c == 'J').count();

    let mut score = match &values[..] {
        [5] => FIVE,
        [1, 4] => FOUR,
        [2, 3] => FULLHOUSE,
        [1, 1, 3] => THREE,
        [1, 2, 2] => TWO,
        [1, 1, 1, 2] => ONE,
        [1, 1, 1, 1, 1] => HIGH,
        _ => unreachable!(),
    };

    score = match (score, j_count) {
        (FOUR, 1 | 4) => FIVE,
        (FULLHOUSE, 2 | 3) => FIVE,
        (THREE, 1 | 3) => FOUR,
        (TWO, 2) => FOUR,
        (TWO, 1) => FULLHOUSE,
        (ONE, 1 | 2) => THREE,
        (HIGH, 1) => ONE,
        _ => score,
    };
    score
}

fn main() {
    let input = include_str!("./input7.txt");
    let mut cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let card = parts[0];
            let bid: i64 = parts[1].parse().unwrap();
            (card.to_owned(), bid)
        })
        .collect();

    cards.sort_unstable_by(|a, b| rank(&a.0, &b.0));

    let ans: i64 = cards
        .iter()
        .enumerate()
        .map(|(idx, c)| (idx as i64 + 1) * c.1)
        .sum();

    println!("{} ", ans);
}
