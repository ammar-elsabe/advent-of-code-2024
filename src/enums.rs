use crate::*;

pub enum Puzzle {
    Puzzle1,
    Puzzle2,
}

pub enum Day {
    Day1(Day1),
    Day2(Day2),
    Day3(Day3),
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
}

impl TryFrom<String> for Puzzle {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "puzzle_1" => Ok(Puzzle::Puzzle1),
            "puzzle_2" => Ok(Puzzle::Puzzle2),
            _ => Err("invalid puzzle".to_owned()),
        }
    }
}

impl TryFrom<i32> for Day {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Day::Day1(Day1)),
            2 => Ok(Day::Day2(Day2)),
            3 => Ok(Day::Day3(Day3)),
            4 => Ok(Day::Day4),
            5 => Ok(Day::Day5),
            6 => Ok(Day::Day6),
            7 => Ok(Day::Day7),
            8 => Ok(Day::Day8),
            9 => Ok(Day::Day9),
            10 => Ok(Day::Day10),
            11 => Ok(Day::Day11),
            12 => Ok(Day::Day12),
            _ => Err("invalid day".to_owned()),
        }
    }
}

impl DoPuzzle for Day {
    fn do_puzzle(self, puzzle: Puzzle, file_name: String) {
        match self {
            Day::Day1(day1) => day1.do_puzzle(puzzle, file_name),
            Day::Day2(day2) => day2.do_puzzle(puzzle, file_name),
            Day::Day3(day3) => day3.do_puzzle(puzzle, file_name),
            _ => unimplemented!(),
        }
    }
}
