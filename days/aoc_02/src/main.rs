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

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}",);
}

fn two_bruteforce(mut input: Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter_mut()
        .filter(|row| {
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
    let elapsed = now.elapsed();
    println!("Two bruteforce: {sum} | Elapsed: {elapsed:?}");
}

fn possible_invalid(row: &[i64]) -> Vec<usize> {
    // Checks for possible invalid positions. When a window fails a check we must
    // consider both numbers as potentially faulty.

    let mut possible_invalid_idx = Vec::with_capacity(3);
    if let Some(idx) = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| {
            let diff = (l - r).abs();
            !(diff >= 1 && diff <= 3)
        })
        .map(|(idx, _)| idx)
    {
        possible_invalid_idx.push(idx);
        possible_invalid_idx.push(idx + 1);

        return possible_invalid_idx;
    }

    if let Some(idx) = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| !(l < r))
        .map(|(idx, _)| idx)
    {
        possible_invalid_idx.push(idx);
        possible_invalid_idx.push(idx + 1);
    }

    if let Some(idx) = row
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .find_position(|[l, r]| !(l > r))
        .map(|(idx, _)| idx)
    {
        possible_invalid_idx.push(idx);
        possible_invalid_idx.push(idx + 1);
    }

    possible_invalid_idx
}

fn two_smart(mut input: Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter_mut()
        .filter(|row| {
            if is_row_valid(row) {
                return true;
            }

            let possible_invalid_indexes = possible_invalid(row);

            for idx in possible_invalid_indexes {
                let first_half = &row[..idx];
                let second_half = &row[idx + 1..];

                if is_row_valid(&[first_half, second_half].concat()) {
                    return true;
                }
            }
            false
        })
        .count();

    let elapsed = now.elapsed();
    println!("Two smart: {sum} | Elapsed: {elapsed:?}",);
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
    two_bruteforce(input.clone());
    two_smart(input);
}
