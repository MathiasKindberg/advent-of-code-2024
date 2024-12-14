#![feature(linked_list_cursors)]

//! Part 1:
//! Part 2:

//! Understanding the input:
//! 12345
//! 1(id=0) 2(free) 3(id = 1) 4(free)  5id(=2)
//! 0..111....22222

// type Input = Vec<Block>;
mod one;
mod two;

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one::one(one::parse(&input).clone());

    two::two(&mut two::parse(&input));
}
