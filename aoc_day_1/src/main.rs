use std::env;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::iter::zip;

enum Puzzle {
    Puzzle1,
    Puzzle2,
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

fn puzzle_1(reader: BufReader<impl std::io::Read>) {
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
    ids_1.sort();
    ids_2.sort();

    let total_distance: i32 = zip(ids_1, ids_2).map(|(id1, id2)| (id1 - id2).abs()).sum();
    println!("total distance: {total_distance}");
}

fn puzzle_2(reader: BufReader<impl std::io::Read>) {
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
            + (next * ids_2_map[TryInto::<usize>::try_into(next - 1).expect("id does not fit in usize")]))
    );
}

fn main() {
    if let (Some(file_name), Some::<Puzzle>(puzzle)) = (
        env::args().nth(1),
        env::args().nth(2).and_then(|puzzle| puzzle.try_into().ok()),
    ) {
        let file = fs::File::open(file_name).expect("error reading input.txt");
        let reader = BufReader::new(file);
        match puzzle {
            Puzzle::Puzzle1 => puzzle_1(reader),
            Puzzle::Puzzle2 => puzzle_2(reader),
        }
    } else {
        println!("usage: <bin> <filename> <puzzle_1/puzzle_2>");
    }
}
