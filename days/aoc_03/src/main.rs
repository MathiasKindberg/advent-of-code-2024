//! Part 1:
//! Part 2:

type Input = Vec<String>;

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let re = regex::Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)").unwrap();
    let operations: Vec<(i64, i64)> = input
        .iter()
        .map(|row| {
            re.captures_iter(row)
                .map(|m| {
                    (
                        m["l"].parse::<i64>().unwrap(),
                        m["r"].parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect();

    let sum = operations.iter().map(|(l, r)| l * r).sum::<i64>();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(input: &Input) {
    let now = std::time::Instant::now();
    let re = regex::Regex::new(r"mul\((?<l>\d+),(?<r>\d+)\)|do\(\)|don\'t\(\)").unwrap();

    let input: Vec<Op> = input
        .iter()
        .map(|row| {
            re.captures_iter(row)
                .map(|m| match m.get(0).unwrap().as_str() {
                    "do()" => Op::Enabled,
                    "don't()" => Op::Disabled,
                    _ => Op::Mul(
                        m["l"].parse::<i64>().unwrap(),
                        m["r"].parse::<i64>().unwrap(),
                    ),
                })
                .collect::<Vec<Op>>()
        })
        .flatten()
        .collect();

    #[derive(Debug)]
    enum Op {
        Enabled,
        Disabled,
        Mul(i64, i64),
    }

    let (_, sum) = input
        .iter()
        .fold((Op::Enabled, 0), |(mut state, mut sum), op| {
            match op {
                Op::Enabled => state = Op::Enabled,
                Op::Disabled => state = Op::Disabled,
                Op::Mul(l, r) => match state {
                    Op::Enabled => sum += l * r,
                    Op::Disabled => (),
                    Op::Mul(_, _) => panic!("Invalid state: {state:?}"),
                },
            }
            (state, sum)
        });

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(&input);
    two(&input);
}
