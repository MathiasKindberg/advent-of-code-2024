//! Part 1:
//! Part 2:

type Input = Vec<(usize, Vec<usize>)>;

// Recursive brute force solution
fn solve_one(final_result: usize, result: usize, row: &[usize]) -> bool {
    if result == final_result {
        return true;
    } else if row.is_empty() {
        return false;
    }

    solve_one(final_result, result + row[0], &row[1..])
        || solve_one(final_result, result * row[0], &row[1..])
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum: usize = input
        .iter()
        .filter_map(|(result, row)| solve_one(*result, row[0], &row[1..]).then_some(result))
        .sum();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// Recursive brute force solution
fn solve_two(final_result: usize, result: usize, row: &[usize]) -> bool {
    if result > final_result {
        return false;
    } else if row.is_empty() {
        return result == final_result;
    }

    solve_two(final_result, result + row[0], &row[1..])
        || solve_two(final_result, result * row[0], &row[1..])
        || solve_two(final_result, concatenate_ints(result, row[0]), &row[1..])
}

fn concatenate_ints(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

fn two(input: &Input) {
    let now = std::time::Instant::now();

    let sum: usize = input
        .iter()
        .filter_map(|(result, row)| solve_two(*result, row[0], &row[1..]).then_some(result))
        .sum();

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| {
            let (left, right) = row.split_once(": ").unwrap();
            (
                left.parse().unwrap(),
                right
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
