//! Part 1:
//! Part 2:

type Input = Vec<usize>;

const fn get_num_digits(num: &usize) -> u32 {
    match num {
        0 => 1,
        _ => num.ilog10() + 1,
    }
}

fn split_int(num: &usize) -> (usize, usize) {
    let num_digits = get_num_digits(num);

    // Integer division and the 10 base used to split the number in half.
    let first = num / 10_usize.pow(num_digits / 2);
    let second = num - first * 10_usize.pow(num_digits / 2);
    (first, second)
}

fn solution(input: &Input, blinks: usize) {
    let now = std::time::Instant::now();

    let mut stones = std::collections::HashMap::new();
    for &num in input {
        *stones.entry(num).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        // Grab all the keys so we can iterate over them.
        // let keys: Vec<_> = stones.keys().cloned().collect();

        // Create a new hashmap to store the updated stone values as we assemble them
        let mut new_stones = std::collections::HashMap::with_capacity(stones.len() * 2);

        for (key, count) in stones {
            if key == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if get_num_digits(&key) % 2 == 0 {
                let (first, second) = split_int(&key);

                *new_stones.entry(first).or_insert(0) += count;
                *new_stones.entry(second).or_insert(0) += count;
            } else {
                *new_stones.entry(key * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }

    let sum = stones.values().sum::<usize>();
    let elapsed = now.elapsed();
    println!("Blinks: {blinks} | Sum: {sum} | Elapsed: {elapsed:?}");
}

// Solution for first part of the problem when the vector size is still manageable.
#[allow(dead_code)]
fn solution_old(mut input: Input, blinks: usize) {
    let now = std::time::Instant::now();

    let mut num_stones = Vec::new();

    for _ in 0..blinks {
        num_stones.push(input.len());

        // A clone each blink but that should be good enough for 25 blinks.
        let mut new_input = Vec::with_capacity(input.len() * 2);

        for num in &input {
            if num == &0 {
                new_input.push(1);
            } else if get_num_digits(num) % 2 == 0 {
                let (first, second) = split_int(num);
                new_input.push(first);
                new_input.push(second);
            } else {
                new_input.push(num * 2024);
            }
        }
        input = new_input;
    }

    let sum = input.len();
    let elapsed = now.elapsed();
    println!("Blinks: {blinks} | Sum: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|row| row.parse().unwrap())
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(input.first().unwrap());

    solution(&input, 25);
    solution(&input, 75);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_int() {
        assert_eq!(split_int(&1234), (12, 34));
        assert_eq!(split_int(&123456), (123, 456));
        assert_eq!(split_int(&12345678), (1234, 5678));
        assert_eq!(split_int(&1000), (10, 0));

        assert!((get_num_digits(&2) % 2 != 0))
    }
}
