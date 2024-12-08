struct Input {
    locations: std::collections::HashMap<char, Vec<(isize, isize)>>,
    max_row: isize,
    max_col: isize,
}

fn get_antinodes(loc_1: (isize, isize), loc_2: (isize, isize)) -> [(isize, isize); 2] {
    let row_dist = loc_2.0 - loc_1.0;
    let col_dist = loc_2.1 - loc_1.1;

    let antinode_1 = (loc_1.0 - row_dist, loc_1.1 - col_dist);
    let antinode_2 = (loc_2.0 + row_dist, loc_2.1 + col_dist);

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

// Now simply extend the line of antinodes until they pass the edge of the grid.
fn get_continous_antinodes_within_bounds(
    loc_1: (isize, isize),
    loc_2: (isize, isize),
    max_row: isize,
    max_col: isize,
) -> Vec<(isize, isize)> {
    // => We have at least two antennas of the same frequency. Thus
    // we need to include their locations.....
    let mut antinodes = vec![loc_1, loc_2];

    let row_dist = loc_2.0 - loc_1.0;
    let col_dist = loc_2.1 - loc_1.1;

    let mut next_row = loc_1.0 - row_dist;
    let mut next_col = loc_1.1 - col_dist;

    while next_row >= 0 && next_row < max_row && next_col >= 0 && next_col < max_col {
        antinodes.push((next_row, next_col));
        next_row -= row_dist;
        next_col -= col_dist;
    }

    let mut next_row = loc_2.0 + row_dist;
    let mut next_col = loc_2.1 + col_dist;

    while next_row >= 0 && next_row < max_row && next_col >= 0 && next_col < max_col {
        antinodes.push((next_row, next_col));
        next_row += row_dist;
        next_col += col_dist;
    }

    antinodes
}

fn two(input: &Input) {
    use itertools::Itertools;
    let now = std::time::Instant::now();

    let mut all_antinodes = std::collections::HashSet::new();

    for (_, locations) in input.locations.iter() {
        for locs in locations.iter().combinations(2) {
            get_continous_antinodes_within_bounds(*locs[0], *locs[1], input.max_row, input.max_col)
                .iter()
                .for_each(|antinode| {
                    all_antinodes.insert(*antinode);
                });
        }
    }

    let sum = all_antinodes.len();
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
