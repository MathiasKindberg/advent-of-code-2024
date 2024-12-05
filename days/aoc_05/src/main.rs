//! Part 1:
//! Part 2:

// type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);
type Input = (std::collections::HashSet<(usize, usize)>, Vec<Vec<usize>>);

fn one(input: &Input) {
    let now = std::time::Instant::now();

    let (rules, input) = input;
    let sum: usize = input
        .iter()
        .filter_map(|row| {
            match row
                .iter()
                .enumerate()
                .all(|(i, n)| row.iter().skip(i + 1).all(|r| rules.contains(&(*n, *r))))
            {
                true => Some(row[row.len() / 2]),
                false => None,
            }
        })
        .sum();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}
fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    let mut input: Vec<String> = input.iter().map(|row| row.to_owned()).collect();
    input.reverse();

    let mut rules = std::collections::HashSet::with_capacity(input.len());

    while let Some(row) = input.pop() {
        if row.is_empty() {
            break;
        }
        rules.insert({
            let (l, r) = row.split_once("|").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        });
    }

    let mut numbers = Vec::with_capacity(input.len());

    while let Some(row) = input.pop() {
        numbers.push(row.split(',').map(|n| n.parse().unwrap()).collect());
    }

    (rules, numbers)
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
