//! Part 1:
//! Part 2:

type Input = Vec<Vec<char>>;

fn one(input: &mut Input) {
    let now = std::time::Instant::now();

    let mut occurrences: usize = 0;
    for _ in 0..4 {
        // Handle horizontal
        let horizontal: usize = input
            .iter()
            .map(|row| {
                row.windows(4)
                    .filter(|window| window == &&['X', 'M', 'A', 'S'])
                    .count()
            })
            .sum::<usize>();

        // Handle diagonal
        let diagonal: usize = input
            .windows(4)
            .map(|input_slice| {
                input_slice[0]
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| {
                        input_slice[0][*i] == 'X'
                            && input_slice[1][i + 1] == 'M'
                            && input_slice[2][i + 2] == 'A'
                            && input_slice[3][i + 3] == 'S'
                    })
                    .count()
            })
            .sum();

        occurrences += horizontal + diagonal;

        aoc_lib::rotate_90_cw_2d(input);
    }

    let elapsed = now.elapsed();
    println!("One: {occurrences} | Elapsed: {elapsed:?}");
}

// Panics if attempting to create a square within closer than 3 characters from the end
fn get_square(window: &mut [Vec<char>], from_pos: usize) -> [&mut [char]; 3] {
    let (row1, remaining) = window.split_at_mut(1);
    let (_, row1_remaining) = row1[0].split_at_mut(from_pos);
    let (row1, _) = row1_remaining.split_at_mut(3);

    let (row2, remaining) = remaining.split_at_mut(1);

    let (_, row2_remaining) = row2[0].split_at_mut(from_pos);
    let (row2, _) = row2_remaining.split_at_mut(3);

    let (row3, _) = remaining.split_at_mut(1);
    let (_, row3_remaining) = row3[0].split_at_mut(from_pos);
    let (row3, _) = row3_remaining.split_at_mut(3);

    [row1, row2, row3]
}

fn two(input: &mut Input) {
    let now = std::time::Instant::now();

    let mut occurrences: usize = 0;

    // Idea: iterate over 3x3 boxes
    // Check if the diagonal is "mas"
    // Roate 4 times
    // If we find 2 mas then we have a x-mas

    // One day we won't need to use a library for this
    use lending_iterator::LendingIterator;
    use lending_iterator::lending_iterator::constructors::windows_mut;

    let mut iter = input.windows_mut::<3>();
    while let Some(window) = iter.next() {
        for i in 0..(window[0].len() - 3) {
            let mut square = get_square(window, i);

            let matches = (0..4)
                .filter(|_| {
                    let diagonal_is_mas =
                        square[0][0] == 'M' && square[1][1] == 'A' && square[2][2] == 'S';
                    aoc_lib::rotate_90_cw_2d_array(&mut square);
                    diagonal_is_mas
                })
                .count();

            if matches >= 2 {
                occurrences += 1;
            }
        }
    }

    let elapsed: std::time::Duration = now.elapsed();
    println!("Two: {occurrences} | Elapsed: {elapsed:?}");
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

    one(&mut input);
    two(&mut input);
}
