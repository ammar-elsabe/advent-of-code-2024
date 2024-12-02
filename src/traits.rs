use crate::Puzzle;

pub trait DoPuzzle {
    fn do_puzzle(self, puzzle: Puzzle, file_name: String);
}
