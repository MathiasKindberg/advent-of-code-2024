//! Part 1:
//! Part 2:

type Input = Vec<Vec<Square>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Obstacle,
    Guard,
    Edge,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            '!' => Self::Edge,
            char => unreachable!("unknown square: {char}"),
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Obstacle => write!(f, "#"),
            Self::Guard => write!(f, "^"),
            Self::Edge => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn find_guard_starting_position(input: &Input) -> (usize, usize) {
    for (y, row) in input.iter().enumerate() {
        for (x, square) in row.iter().enumerate() {
            if square == &Square::Guard {
                return (x, y);
            }
        }
    }
    unreachable!("no guard found")
}

fn next_position(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Visited {
    Yes,
    No,
}

impl std::fmt::Display for Visited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "Y"),
            Self::No => write!(f, "N"),
        }
    }
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let (mut x, mut y) = find_guard_starting_position(input);
    let mut direction = Direction::Up;
    let mut visited: Vec<Vec<Visited>> = vec![vec![Visited::No; input[0].len()]; input.len()];

    while input[y][x] != Square::Edge {
        visited[y][x] = Visited::Yes;
        let (next_x, next_y) = next_position(x, y, &direction);
        // Doesn't work because now we are standing on an obstacle
        if input[next_y][next_x] == Square::Obstacle {
            direction = direction.turn_right();
        }
        match direction {
            Direction::Up => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        }
    }

    let sum = visited
        .iter()
        .flatten()
        .filter(|v| **v == Visited::Yes)
        .count();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// Takes 2s to run. Could be made more efficient by only checking the visited squares from part 1.
fn two(mut input: Input) {
    let now = std::time::Instant::now();
    let (mut x, mut y) = find_guard_starting_position(&input);
    let guard_x = x;
    let guard_y = y;

    let mut direction = Direction::Up;

    let mut visited: std::collections::HashSet<(usize, usize, Direction)> =
        std::collections::HashSet::new();

    let mut num_cycles = 0;

    for obstacle_y in 1..input.len() - 1 {
        for obstacle_x in 1..input[0].len() - 1 {
            // Skip the guard
            if obstacle_y == guard_y && obstacle_x == guard_x {
                continue;
            }
            let original_square = input[obstacle_y][obstacle_x];
            input[obstacle_y][obstacle_x] = Square::Obstacle;

            while input[y][x] != Square::Edge {
                if !visited.insert((x, y, direction)) {
                    num_cycles += 1;
                    break;
                }

                loop {
                    let (next_x, next_y) = next_position(x, y, &direction);
                    if input[next_y][next_x] == Square::Obstacle {
                        direction = direction.turn_right();
                    } else {
                        break;
                    }
                }

                match direction {
                    Direction::Up => y -= 1,
                    Direction::Right => x += 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                }
            }

            // Reset visited, obstacle and starting location
            visited.clear();
            input[obstacle_y][obstacle_x] = original_square;
            x = guard_x;
            y = guard_y;
            direction = Direction::Up;
        }
    }

    let elapsed = now.elapsed();
    println!("Two: {num_cycles} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| row.chars().map(|c| c.into()).collect())
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    let input = aoc_lib::pad_input(input, Square::Edge);

    one(&input);
    two(input);
}
