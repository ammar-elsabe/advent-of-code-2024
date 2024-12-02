use std::env;
mod enums;
use enums::*;
mod days;
use days::*;
mod traits;
use traits::*;

fn main() {
    if let (Some(file_name), Some::<Day>(day), Some::<Puzzle>(puzzle)) = (
        env::args().nth(1),
        env::args()
            .nth(2)
            .and_then(|day| day.parse::<i32>().ok())
            .and_then(|day| day.try_into().ok()),
        env::args().nth(3).and_then(|puzzle| puzzle.try_into().ok()),
    ) {
        day.do_puzzle(puzzle, file_name);
    } else {
        println!("usage: <bin> <filename> <day> <puzzle_1/puzzle_2>");
    }
}
