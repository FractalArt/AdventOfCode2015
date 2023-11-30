//! # Advent of Code 2015 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2015/day/1).

/// The solution to task 1 of day 1.
pub fn day_01_1(data: &str) -> i32 {
    data.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

/// The solution to task 2 of day 1
pub fn day_01_2(data: &str) -> usize {
    data.chars()
        .scan(0, |state, x| {
            *state += if x == '(' { 1 } else { -1 };
            if *state < 0 {
                None
            } else {
                Some(*state)
            }
        })
        .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_1() {
        assert_eq!(day_01_1("(())"), 0);
        assert_eq!(day_01_1("()()"), 0);
        assert_eq!(day_01_1("((("), 3);
        assert_eq!(day_01_1("(()(()("), 3);
        assert_eq!(day_01_1("(()(()("), 3);
        assert_eq!(day_01_1("())"), -1);
        assert_eq!(day_01_1("))("), -1);
        assert_eq!(day_01_1(")))"), -3);
        assert_eq!(day_01_1(")())())"), -3);
    }

    #[test]
    fn test_day_01_2() {
        assert_eq!(day_01_2("()())"), 5);
    }
}
