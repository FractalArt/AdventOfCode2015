//! # Advent of Code 2015 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2015/day/2).
use itertools::Itertools;

/// The solution to task 1 of day 2.
pub fn day_02_1(data: &[String]) -> u32 {
    data.iter()
        .map(|s| s.split('x').map(|i| i.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(|v| vec![v[0]*v[1], v[1]*v[2], v[2]*v[0]])
        .map(|v| v.iter().min().unwrap() + 2u32 * v.iter().sum::<u32>())
        .sum()
}

/// The solution to task 2 of day 2.
pub fn day_02_2(data: &[String]) -> u32 {
    data.iter()
        .map(|s| s.split('x').map(|i| i.parse::<u32>().unwrap()).sorted())
        .map(|v| v.clone().take(2).sum::<u32>() * 2 + v.product::<u32>())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_1() {
        assert_eq!(day_02_1(&["2x3x4".to_string()]), 58);
        assert_eq!(day_02_1(&["1x1x10".to_string()]), 43);
    }

    #[test]
    fn test_day_02_2() {
        assert_eq!(day_02_2(&["2x3x4".to_string()]), 34);
        assert_eq!(day_02_2(&["1x1x10".to_string()]), 14);
    }

}
