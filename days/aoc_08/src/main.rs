//! Part 1:
//! Part 2:
struct Input {
    locations: std::collections::HashMap<char, Vec<(isize, isize)>>,
    max_row: isize,
    max_col: isize,
}

// fn get_antinodes(loc_1: (isize, isize), loc_2: (isize, isize)) -> [(isize, isize); 2] {
fn get_antinodes(loc_1: (isize, isize), loc_2: (isize, isize)) -> [(isize, isize); 2] {
    // Order the input to ensure the topmost coordinate comes first.
    let mut locations = [&loc_1, &loc_2];
    locations.sort();

    let row_dist = locations[1].0 - locations[0].0;
    let col_dist = locations[1].1 - locations[0].1;

    let antinode_1 = (locations[0].0 - row_dist, locations[0].1 - col_dist);
    let antinode_2 = (locations[1].0 + row_dist, locations[1].1 + col_dist);

    [antinode_1, antinode_2]
}

fn one(input: &Input) {
    use itertools::Itertools;
    let now = std::time::Instant::now();

    let mut all_antinodes = std::collections::HashSet::new();

    for (_, locations) in input.locations.iter() {
        for locs in locations.iter().combinations(2) {
            get_antinodes(*locs[0], *locs[1])
                .iter()
                .filter(|antinode| {
                    antinode.0 >= 0
                        && antinode.0 < input.max_row
                        && antinode.1 >= 0
                        && antinode.1 < input.max_col
                })
                .for_each(|antinode| {
                    all_antinodes.insert(*antinode);
                });
        }
    }

    let sum = all_antinodes.len();
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
    let max_row = input.len() as isize;
    let max_col = input[0].len() as isize;

    let locations = input.iter().enumerate().fold(
        std::collections::HashMap::new(),
        |mut acc: std::collections::HashMap<char, Vec<(isize, isize)>>, (row, input)| {
            input.chars().enumerate().for_each(|(col, c)| match c {
                '.' => (),
                _ => acc.entry(c).or_default().push((row as isize, col as isize)),
            });
            acc
        },
    );

    Input {
        locations,
        max_row,
        max_col,
    }
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    one(&input);
    two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_antinodes_commutative() {
        // Antinodes should not depend on order. They are simply extensions of the line between the two points
        let mut antinodes_1 = get_antinodes((3, 3), (5, 5));
        let mut antinodes_2 = get_antinodes((5, 5), (3, 3));

        antinodes_1.sort();
        antinodes_2.sort();

        assert_eq!(antinodes_1, antinodes_2);

        let mut antinodes_1 = get_antinodes((5, 3), (3, 5));
        let mut antinodes_2 = get_antinodes((3, 5), (5, 3));

        antinodes_1.sort();
        antinodes_2.sort();

        assert_eq!(antinodes_1, antinodes_2);

        let mut antinodes_1 = get_antinodes((3, 3), (3, 5));
        let mut antinodes_2 = get_antinodes((3, 5), (3, 3));

        antinodes_1.sort();
        antinodes_2.sort();
        assert_eq!(antinodes_1, antinodes_2);
    }
}
