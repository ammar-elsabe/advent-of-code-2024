use std::{
    fs,
    io::{prelude::*, BufReader},
};

use crate::{DoPuzzle, Puzzle};

pub struct Day2;

impl Day2 {
    fn read_file(file_name: String) -> Vec<Vec<i32>> {
        let file = fs::File::open(file_name).expect("error reading input.txt");
        let reader = BufReader::new(file);
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines().map(|line| line.expect("error reading line")) {
            let levels = line
                .split(' ')
                .map(|level| {
                    level
                        .parse::<i32>()
                        .expect(format!("invalid level: {}", level).as_str())
                })
                .collect::<Vec<_>>();
            reports.push(levels);
        }
        reports
    }

    fn is_safe(report: &Vec<i32>) -> bool {
        let mut difference = report[1] - report[0];
        if difference.abs() > 3 || difference.abs() < 1 {
            return false;
        }
        let direction = difference.is_positive();
        for i in 2..report.len() {
            difference = report[i] - report[i - 1];
            if difference.abs() > 3 || difference.abs() < 1 {
                return false;
            }
            let new_direction = difference.is_positive();
            if new_direction != direction {
                return false;
            }
        }
        true
    }

    fn is_safe_v2(report: Vec<i32>) -> bool {
        //let mut difference = report[1] - report[0];
        //if difference.abs() > 3 || difference.abs() < 1 {
        //    let mut report_copy = report.clone();
        //    report_copy.remove(0);
        //    if Self::is_safe(&report_copy) {
        //        return true;
        //    } else {
        //        report.remove(1);
        //        return Self::is_safe(&report);
        //    }
        //}
        //let direction = difference.is_positive();
        //for i in 2..report.len() {
        //    difference = report[i] - report[i - 1];
        //    if difference.abs() > 3 || difference.abs() < 1 {
        //        let mut report_copy = report.clone();
        //        report_copy.remove(i - 1);
        //        if Self::is_safe(&report_copy) {
        //            return true;
        //        } else {
        //            report.remove(i);
        //            return Self::is_safe(&report);
        //        }
        //    }
        //    let new_direction = difference.is_positive();
        //    if new_direction != direction {
        //        let mut report_copy = report.clone();
        //        report_copy.remove(i - 1);
        //        if Self::is_safe(&report_copy) {
        //            return true;
        //        } else {
        //            report.remove(i);
        //            return Self::is_safe(&report);
        //        }
        //    }
        //}
        //true
        if Self::is_safe(&report) {
            true
        } else {
            (0..report.len())
                .map(|i| {
                    report
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, e)| if idx != i { Some(e.clone()) } else { None })
                        .collect::<Vec<_>>()
                })
                .any(|rep| Self::is_safe(&rep))
        }
    }

    fn puzzle_1(reports: Vec<Vec<i32>>) -> usize {
        let ret = reports.into_iter().filter(Day2::is_safe).count();
        println!("safe reports: {ret}");
        ret
    }

    fn puzzle_2(reports: Vec<Vec<i32>>) -> usize {
        let ret = reports
            .into_iter()
            .map(Self::is_safe_v2)
            .filter(Clone::clone)
            .count();
        println!("safe reports: {ret}");
        ret
    }
}

impl DoPuzzle for Day2 {
    fn do_puzzle(self, puzzle: crate::Puzzle, file_name: String) {
        let reports = Day2::read_file(file_name);
        match puzzle {
            Puzzle::Puzzle1 => Day2::puzzle_1(reports),
            Puzzle::Puzzle2 => Day2::puzzle_2(reports),
        };
    }
}

mod tests {
    use super::*;

    #[test]
    fn puzzle_1() {
        let actual = Day2::puzzle_1(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn puzzle_2() {
        let actual = Day2::puzzle_2(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
        assert_eq!(actual, 4);
    }
}
