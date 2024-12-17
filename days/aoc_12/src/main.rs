//! Part 1:
//! Part 2:

type Input = Vec<Vec<char>>;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn perimeter_count(&self, input: &Input) -> usize {
        let mut perimeter_count = 0;
        let c = input[self.row][self.col];

        if input[self.row - 1][self.col] != c {
            perimeter_count += 1;
        }
        if input[self.row + 1][self.col] != c {
            perimeter_count += 1;
        }
        if input[self.row][self.col - 1] != c {
            perimeter_count += 1;
        }
        if input[self.row][self.col + 1] != c {
            perimeter_count += 1;
        }

        perimeter_count
    }

    fn corner_count(&self, input: &Input) -> usize {
        let mut corner_count = 0;
        let c = input[self.row][self.col];

        // Check whether we have an outside corner or inside corner, and then make sure we
        // are not in the middle of a large blob by checking the diagonals.

        // Top left corner
        if (input[self.row - 1][self.col] == c
            && input[self.row][self.col - 1] == c
            && input[self.row - 1][self.col - 1] != c)
            || (input[self.row - 1][self.col] != c && input[self.row][self.col - 1] != c)
        {
            corner_count += 1;
        }

        // Top right corner
        if (input[self.row - 1][self.col] == c
            && input[self.row][self.col + 1] == c
            && input[self.row - 1][self.col + 1] != c)
            || (input[self.row - 1][self.col] != c && input[self.row][self.col + 1] != c)
        {
            corner_count += 1;
        }

        // Bottom right corner
        if (input[self.row + 1][self.col] == c
            && input[self.row][self.col + 1] == c
            && input[self.row + 1][self.col + 1] != c)
            || (input[self.row + 1][self.col] != c && input[self.row][self.col + 1] != c)
        {
            corner_count += 1;
        }

        // Bottom left corner
        if (input[self.row + 1][self.col] == c
            && input[self.row][self.col - 1] == c
            && input[self.row + 1][self.col - 1] != c)
            || (input[self.row + 1][self.col] != c && input[self.row][self.col - 1] != c)
        {
            corner_count += 1;
        }

        corner_count
    }
}

#[derive(Debug)]
struct BfsResult {
    area: usize,
    perimeter: usize,
    corner: usize,
}

fn bfs(input: &Input, curr: Point, visited: &mut Vec<Vec<bool>>) -> BfsResult {
    let mut area = 1;
    let mut perimeter = curr.perimeter_count(input);
    // The corner count is the number of sides.
    let mut corner = curr.corner_count(input);

    let c = input[curr.row][curr.col];
    visited[curr.row][curr.col] = true;

    if input[curr.row - 1][curr.col] == c && !visited[curr.row - 1][curr.col] {
        let bfs_result = bfs(
            input,
            Point {
                row: curr.row - 1,
                col: curr.col,
            },
            visited,
        );
        area += bfs_result.area;
        perimeter += bfs_result.perimeter;
        corner += bfs_result.corner;
    }

    if input[curr.row + 1][curr.col] == c && !visited[curr.row + 1][curr.col] {
        let bfs_result = bfs(
            input,
            Point {
                row: curr.row + 1,
                col: curr.col,
            },
            visited,
        );
        area += bfs_result.area;
        perimeter += bfs_result.perimeter;
        corner += bfs_result.corner;
    }

    if input[curr.row][curr.col - 1] == c && !visited[curr.row][curr.col - 1] {
        let bfs_result = bfs(
            input,
            Point {
                row: curr.row,
                col: curr.col - 1,
            },
            visited,
        );
        area += bfs_result.area;
        perimeter += bfs_result.perimeter;
        corner += bfs_result.corner;
    }

    if input[curr.row][curr.col + 1] == c && !visited[curr.row][curr.col + 1] {
        let bfs_result = bfs(
            input,
            Point {
                row: curr.row,
                col: curr.col + 1,
            },
            visited,
        );
        area += bfs_result.area;
        perimeter += bfs_result.perimeter;
        corner += bfs_result.corner;
    }

    BfsResult {
        area,
        perimeter,
        corner,
    }
}

fn solve(input: &Input) {
    let now = std::time::Instant::now();

    // A HashSet is neater but vector is faster since we have nice easy bounds for the data
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut results = vec![];

    for (row, row_data) in input.iter().enumerate() {
        for (col, &c) in row_data.iter().enumerate() {
            if c == '.' || visited[row][col] {
                continue;
            }

            let bfs_result = bfs(input, Point { row, col }, &mut visited);
            results.push((c, bfs_result));
        }
    }

    let sum_one = results
        .iter()
        .map(|(_, bfs_result)| bfs_result.area * bfs_result.perimeter)
        .sum::<usize>();
    let sum_two = results
        .iter()
        .map(|(_, bfs_result)| bfs_result.area * bfs_result.corner)
        .sum::<usize>();

    let elapsed = now.elapsed();
    println!("One: {sum_one} | Two: {sum_two} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input.iter().map(|row| row.chars().collect()).collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    let input = aoc_lib::pad_input(input, '.');

    solve(&input);
}
