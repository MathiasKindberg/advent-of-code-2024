#[derive(Debug)]
enum Status {
    Free,
    Id(usize),
}

// Fast enough but not very efficient when inserting new blocks in the middle.
pub(crate) type Input = Vec<Block>;

#[derive(Debug)]
pub(crate) struct Block {
    status: Status,
    length: usize,
}

pub(crate) fn two(input: &mut Input) {
    let now = std::time::Instant::now();
    let mut curr_block = input.len();

    while curr_block > 0 {
        curr_block -= 1;

        match input[curr_block].status {
            Status::Free => {
                continue;
            }
            Status::Id(_) => (),
        }

        let length = input[curr_block].length;

        let Some(first_free_block_index) = input
            .iter()
            .position(|block| matches!(block.status, Status::Free) && block.length >= length)
        else {
            continue;
        };

        let remaining_space = input[first_free_block_index].length - length;

        if first_free_block_index < curr_block {
            input.swap(curr_block, first_free_block_index);
            if remaining_space > 0 {
                // With the free block being longer than the current block we need to ensure that only
                // the needed parts of the free space is moved to where the current block came from.
                input[curr_block].length -= remaining_space;

                // Need to consider the case if we insert a free block next to an already existing
                // free block. Which means we should grow it and not create two free blocks.
                if matches!(input[first_free_block_index + 1].status, Status::Free) {
                    input[first_free_block_index + 1].length += remaining_space;
                } else {
                    input.insert(first_free_block_index + 1, Block {
                        status: Status::Free,
                        length: remaining_space,
                    });
                }
            }
        }
    }
    // vec![id; b.length]
    let input: Vec<_> = input
        .iter()
        .flat_map(|b| {
            (0..b.length).map(move |_| match b.status {
                Status::Free => 0,
                Status::Id(id) => id,
            })
        })
        .collect();

    let sum = input.iter().enumerate().map(|(i, b)| i * b).sum::<usize>();
    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            Status::Free => write!(f, "{}", (0..self.length).map(|_| ".").collect::<String>()),
            Status::Id(id) => write!(
                f,
                "{}",
                (0..self.length).map(|_| id.to_string()).collect::<String>()
            ),
        }
    }
}

pub(crate) fn parse(input: &[String]) -> Input {
    let input = input.iter().next().unwrap();
    let mut id = 0;
    let mut is_file = true;
    let mut res = Vec::new();

    for digit in input.chars().map(|c| c.to_digit(10).unwrap()) {
        let status = if is_file {
            Status::Id(id)
        } else {
            Status::Free
        };
        res.push(Block {
            status,
            length: digit as usize,
        });
        id = if is_file { id + 1 } else { id };
        is_file = !is_file
    }

    res.into_iter().collect()
}

#[allow(unused)]
fn print_input(input: &Input) {
    println!(
        "{}",
        input.iter().map(|b| b.to_string()).collect::<String>()
    );
}
