// Utilizing options so we can O(1) take and reposition elements.
type Input = Vec<Option<usize>>;

pub(crate) fn one(mut input: Input) {
    let now = std::time::Instant::now();

    let mut first_free_index = input
        .iter()
        .enumerate()
        .find(|(_, b)| b.is_none())
        .unwrap()
        .0;

    let mut last_occupied = input.len() - 1;

    while first_free_index <= last_occupied {
        let block = input.get_mut(last_occupied).unwrap().take();
        input[first_free_index] = block;

        while input[first_free_index].is_some() {
            first_free_index += 1;
        }

        while input[last_occupied].is_none() {
            last_occupied -= 1;
        }
    }

    let sum = input
        .iter()
        .enumerate()
        .map(|(i, b)| i * b.unwrap_or(0))
        .sum::<usize>();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

pub(crate) fn parse(input: &[String]) -> Input {
    let input = input.iter().next().unwrap();
    let mut id = 0;
    let mut is_file = true;
    let mut res = Vec::with_capacity(input.len());

    for digit in input.chars().map(|c| c.to_digit(10).unwrap()) {
        for _ in 0..digit {
            if is_file {
                res.push(Some(id));
            } else {
                res.push(None);
            }
        }
        if is_file {
            id += 1
        }
        is_file = !is_file
    }
    res
}
