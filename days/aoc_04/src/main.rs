//! Part 1:
//! Part 2:

type Input = Vec<Vec<char>>;

fn one(input: &mut Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    // Handle vertical and horizontal
    for _ in 0..3 {
        aoc_lib::rotate_90_cw_2d(input);
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}
fn two(input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input.iter().map(|row| row.chars().collect()).collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    let mut input = aoc_lib::pad_input(input, '.');

    aoc_lib::print_2d(&input);

    one(&mut input);
    two(&input);
}
