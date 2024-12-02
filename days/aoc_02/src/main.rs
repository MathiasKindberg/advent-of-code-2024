//! Part 1:
//! Part 2:

use core::panic;
use std::io::BufRead;

type Input = Vec<Vec<i64>>;

fn is_row_valid(row: &[i64]) -> bool {
    (row.is_sorted_by(|a, b| a > b) || row.is_sorted_by(|a, b| a < b))
        && row.windows(2).all(|w| w[0].max(w[1]) - w[1].min(w[0]) <= 3)
}

fn one(input: &Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter()
        .filter(|row| {
            is_row_valid(&row)
            // // Ensure that the levels are either all decreasing or increasing
            // if !(row.is_sorted_by(|a, b| a > b) || row.is_sorted_by(|a, b| a < b)) {
            //     return None;
            // }
            // // Ensure that the difference between adjacent levels is at most 3
            // if !row.windows(2).all(|w| w[0].max(w[1]) - w[1].min(w[0]) <= 3) {
            //     return None;
            // }
            // Some(())
        })
        .count();

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(mut input: Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter_mut()
        .filter(|row| {
            if is_row_valid(row) {
                return true;
            }

            // Bruteforce checking if we can remove any index to make it valid
            for i in 0..row.len() {
                let first_half = &row[..i];
                let second_half = &row[i + 1..];

                if is_row_valid(&[first_half, second_half].concat()) {
                    return true;
                }
            }

            false
        })
        .count();

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    // Keep input owned by main function to allow nifty references.
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(input);
}
