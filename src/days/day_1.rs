use std::{
    fs,
    io::{prelude::*, BufReader},
    iter::zip,
};

use crate::{DoPuzzle, Puzzle};

pub struct Day1;

impl Day1 {
    fn read_file(file_name: String) -> (Vec<i32>, Vec<i32>) {
        let file = fs::File::open(file_name).expect("error reading input.txt");
        let reader = BufReader::new(file);
        let mut ids_1 = Vec::new();
        let mut ids_2 = Vec::new();
        for line in reader.lines().map(|line| line.expect("error reading line")) {
            if let [id1, id2] = line
                .split(' ')
                .map(|id| id.parse::<i32>())
                .fold(Vec::<i32>::new(), |mut cum, next| {
                    if let Ok(id) = next {
                        cum.push(id);
                    }
                    cum
                })
                .as_slice()
            {
                ids_1.push(id1.clone());
                ids_2.push(id2.clone());
            } else {
                panic!("line does not match expected pattern");
            }
        }
        (ids_1, ids_2)
    }
    fn puzzle_1(mut ids_1: Vec<i32>, mut ids_2: Vec<i32>) {
        ids_1.sort();
        ids_2.sort();

        let total_distance: i32 = zip(ids_1, ids_2).map(|(id1, id2)| (id1 - id2).abs()).sum();
        println!("total distance: {total_distance}");
    }

    fn puzzle_2(ids_1: Vec<i32>, ids_2: Vec<i32>) {
        let mut ids_2_map = vec![
            0;
            ids_2
                .iter()
                .map(|i| i.clone())
                .max()
                .unwrap_or(0)
                .try_into()
                .expect("id does not fit in usize")
        ];

        for id in ids_2.into_iter() {
            ids_2_map[TryInto::<usize>::try_into(id - 1).expect("id does not fit in usize")] += 1;
        }

        println!(
            "similarity score: {}",
            ids_1.into_iter().fold(0, |cum, next| cum
                + (next
                    * ids_2_map[TryInto::<usize>::try_into(next - 1)
                        .expect("id does not fit in usize")]))
        );
    }
}

impl DoPuzzle for Day1 {
    fn do_puzzle(self, puzzle: crate::Puzzle, file_name: String) {
        let (ids_1, ids_2) = Day1::read_file(file_name);
        match puzzle {
            Puzzle::Puzzle1 => Day1::puzzle_1(ids_1, ids_2),
            Puzzle::Puzzle2 => Day1::puzzle_2(ids_1, ids_2),
        };
    }
}
