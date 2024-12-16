//! Part 1:
//! Part 2:

#[derive(Debug, Clone)]
enum Tile {
    Height(usize),
    Wall,
}
type Input = Vec<Vec<Tile>>;

impl Tile {
    fn is_one_higher(&self, curr_height: usize) -> bool {
        match self {
            Tile::Height(h) => *h == curr_height + 1,
            Tile::Wall => false,
        }
    }

    fn unwrap_height(&self) -> usize {
        match self {
            Tile::Height(h) => *h,
            Tile::Wall => panic!("Should now be able to stand in a wall"),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Loc {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    start: Loc,
    curr: Loc,
}

fn one_and_two(input: &Input) {
    let now = std::time::Instant::now();

    let find_starts: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, tile)| match tile {
                    Tile::Height(0) => Some(State {
                        start: Loc {
                            row: row_idx,
                            col: col_idx,
                        },
                        curr: Loc {
                            row: row_idx,
                            col: col_idx,
                        },
                    }),
                    _ => None,
                })
        })
        .collect();

    let mut queue = std::collections::VecDeque::from(find_starts);
    let mut paths = std::collections::HashMap::new();

    // Last time we recursed this time we queue BFS style.
    while let Some(state) = queue.pop_front() {
        let curr_height = input[state.curr.row][state.curr.col].unwrap_height();
        if curr_height == 9 {
            paths.entry(state).and_modify(|e| *e += 1).or_insert(1);
            continue;
        }

        if input[state.curr.row + 1][state.curr.col].is_one_higher(curr_height) {
            queue.push_back(State {
                start: state.start,
                curr: Loc {
                    row: state.curr.row + 1,
                    col: state.curr.col,
                },
            });
        }
        if input[state.curr.row][state.curr.col + 1].is_one_higher(curr_height) {
            queue.push_back(State {
                start: state.start,
                curr: Loc {
                    row: state.curr.row,
                    col: state.curr.col + 1,
                },
            });
        }
        if input[state.curr.row - 1][state.curr.col].is_one_higher(curr_height) {
            queue.push_back(State {
                start: state.start,
                curr: Loc {
                    row: state.curr.row - 1,
                    col: state.curr.col,
                },
            });
        }
        if input[state.curr.row][state.curr.col - 1].is_one_higher(curr_height) {
            queue.push_back(State {
                start: state.start,
                curr: Loc {
                    row: state.curr.row,
                    col: state.curr.col - 1,
                },
            });
        }
    }

    let elapsed = now.elapsed();
    println!("One: {} | Elapsed: {elapsed:?}", paths.len());
    println!(
        "Two: {} | Elapsed: {elapsed:?}",
        paths.values().sum::<usize>()
    );
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Tile::Wall,
                    c => Tile::Height(c.to_digit(10).unwrap() as usize),
                })
                .collect()
        })
        .collect()
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Height(h) => write!(f, "{h}"),
            Tile::Wall => write!(f, "."),
        }
    }
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    let input = aoc_lib::pad_input(input, Tile::Wall);
    one_and_two(&input);
}
