//! Part 1:
//! Part 2:

use std::io::BufRead;

type Input = (Vec<i64>, Vec<i64>);

fn one(input: Input) {
    let now = std::time::Instant::now();

    let (mut left, mut right) = input;
    left.sort();
    right.sort();

    let sum: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(input: &Input) {
    use std::collections::HashMap;

    let now = std::time::Instant::now();
    let (left, right) = input;
    let frequencies = right.iter().fold(HashMap::new(), |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });

    let sum: i64 = left
        .iter()
        .map(|item| item * *frequencies.get(item).unwrap_or(&0))
        .sum();

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn parse(input: &[String]) -> Input {
    use itertools::Itertools;

    input
        .iter()
        .map(|row| {
            row.split("   ")
                .map(|item| item.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn main() {
    // Keep input owned by main function to allow nifty references.
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(input.clone());
    two(&input);
}
