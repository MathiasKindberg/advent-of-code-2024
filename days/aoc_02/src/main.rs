//! Part 1:
//! Part 2:

use std::io::BufRead;

use itertools::Itertools;

type Input = Vec<Vec<i64>>;

fn is_row_valid(row: &[i64]) -> bool {
    // Two consecutive numbers are handled by the strict ordering.
    (row.is_sorted_by(|a, b| a > b) || row.is_sorted_by(|a, b| a < b))
        && row.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum = input.iter().filter(|row| is_row_valid(&row)).count();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn first_invalid(row: &[i64]) -> Option<usize> {
    // Correct difference
    if let Some(idx) = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| {
            let diff = (l - r).abs();
            !(diff >= 1 && diff <= 3)
        })
        .map(|(idx, _)| idx + 1)
    {
        return Some(idx);
    }

    let strictly_increasing_fail_idx = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| !(l < r))
        .map(|(idx, _)| idx + 1);

    let strictly_decreasing_fail_idx = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| !(l > r))
        .map(|(idx, _)| idx + 1);

    if strictly_increasing_fail_idx.is_none() || strictly_decreasing_fail_idx.is_none() {
        return None;
    }

    match (strictly_increasing_fail_idx, strictly_decreasing_fail_idx) {
        (None, None) => None,
        (Some(_), None) => strictly_increasing_fail_idx,
        (None, Some(_)) => strictly_decreasing_fail_idx,
        (Some(failed_increasing), Some(failed_decreasing)) => {
            Some(failed_increasing.min(failed_decreasing))
        }
    }
}

fn two(mut input: Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter_mut()
        .filter(|row| {
            let first_invalid_idx = first_invalid(row);
            println!("{row:?} With invalid index: {:?}", first_invalid_idx);
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
