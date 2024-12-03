use std::fs;

use crate::{DoPuzzle, Puzzle};

pub struct Day3;

impl Day3 {
    fn read_file(file_name: String) -> String {
        fs::read_to_string(file_name).expect("error reading file")
    }

    fn puzzle_1(input_string: String) -> i32 {
        let mut search_space = input_string.as_str();
        let mut ret: i32 = 0;
        while let Some((opening, substr)) = search_space.find("mul(").and_then(|opening| {
            search_space[opening..search_space.len()]
                .find(')')
                .map(|closing| (opening + 4)..(closing + opening))
                .and_then(|range| search_space.get(range))
                .map(|substr| (opening, substr))
        }) {
            if let [Some(num1), Some(num2)] = substr
                .split(',')
                .map(|numstr| {
                    if numstr.len() > 3 || numstr.len() < 1 {
                        None
                    } else {
                        numstr.parse::<i32>().ok()
                    }
                })
                .collect::<Vec<_>>()
                .as_slice()
            {
                ret += num1 * num2;
            }
            search_space = &search_space[(opening + 4)..search_space.len()]
        }
        println!("sum of multiplications: {ret}");
        ret
    }
}

impl DoPuzzle for Day3 {
    fn do_puzzle(self, puzzle: Puzzle, file_name: String) {
        let file_contents = Self::read_file(file_name);
        match puzzle {
            Puzzle::Puzzle1 => Self::puzzle_1(file_contents),
            Puzzle::Puzzle2 => unimplemented!(),
        };
    }
}

mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        let actual = Day3::puzzle_1(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_owned(),
        );
        assert_eq!(actual, 161);
    }

    #[test]
    fn puzzle_2() {}
}
