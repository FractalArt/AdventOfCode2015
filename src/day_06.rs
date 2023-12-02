//! # Advent of Code 2015 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2015/day/6).
use ndarray::prelude::*;
use ndarray::Array2;
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
}

fn parse_instruction(instruction: &str) -> (&str, usize, usize, usize, usize) {
    let captures = RE.captures(instruction).unwrap();
    (
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(5).unwrap().as_str().parse::<usize>().unwrap(),
    )
}

/// The solution to task 1 of day 5.
pub fn day_06_1(data: &[String]) -> isize {
    data.iter()
        .fold(Array2::zeros((1000, 1000)), |mut grid, ins| {
            let (action, x1, y1, x2, y2) = parse_instruction(ins);
            match action {
                "turn on" => grid.slice_mut(s![x1..=x2, y1..=y2]).fill(1isize),
                "turn off" => grid.slice_mut(s![x1..=x2, y1..=y2]).fill(0),
                _ => grid
                    .slice_mut(s![x1..=x2, y1..=y2])
                    .mapv_inplace(|x| (x - 1).abs()),
            };
            grid
        })
        .sum()
}

/// The solution to task 2 of day 5.
pub fn day_06_2(data: &[String]) -> isize {
    data.iter()
        .fold(Array2::zeros((1000, 1000)), |mut grid, ins| {
            let (action, x1, y1, x2, y2) = parse_instruction(ins);
            match action {
                "turn on" => grid.slice_mut(s![x1..=x2, y1..=y2]).mapv_inplace(|x| x + 1),
                "turn off" => grid
                    .slice_mut(s![x1..=x2, y1..=y2])
                    .mapv_inplace(|x| std::cmp::min(x - 1, 0)),
                _ => grid.slice_mut(s![x1..=x2, y1..=y2]).mapv_inplace(|x| x + 2),
            };
            grid
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_1() {
        let data = [
            "turn on 0,0 through 999,999".to_string(),
            "toggle 0,0 through 999,0".to_string(),
            "turn off 499,499 through 500,500".to_string(),
        ];
        assert_eq!(day_06_1(&data), 1000 * 1000 - 1000 - 4);
    }

    #[test]
    fn test_day_06_2() {
        let data_1 = ["turn on 0,0 through 0,0".to_string()];
        let data_2 = ["toggle 0,0 through 999,999".to_string()];

        assert_eq!(day_06_2(&data_1), 1);
        assert_eq!(day_06_2(&data_2), 2_000_000);
    }
}
