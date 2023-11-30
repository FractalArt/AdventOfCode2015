//! # Advent of Code 2015 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2015/day/3).
use std::collections::HashSet;

/// The solution to task 1 of day 3.
pub fn day_03_1(data: &str) -> usize {
    data.chars()
        .fold(
            (0, 0, vec![(0, 0)].into_iter().collect::<HashSet<_>>()),
            |mut state, c| {
                match c {
                    '^' => state.1 += 1,
                    '>' => state.0 += 1,
                    '<' => state.0 -= 1,
                    _ => state.1 -= 1,
                };
                state.2.insert((state.0, state.1));
                state
            },
        )
        .2
        .len()
}

/// The solution to task 2 of day 3.
pub fn day_03_2(data: &str) -> usize {
    data.chars()
        .enumerate()
        .fold(
            (
                vec![(0, 0), (0, 0)],
                vec![(0, 0)].into_iter().collect::<HashSet<_>>(),
            ),
            |(mut santa_robo, mut hs), (step, c)| {
                match c {
                    '>' => santa_robo[step % 2].0 += 1,
                    '<' => santa_robo[step % 2].0 -= 1,
                    '^' => santa_robo[step % 2].1 += 1,
                    _ => santa_robo[step % 2].1 -= 1,
                };
                hs.insert(santa_robo[step % 2]);
                (santa_robo, hs)
            },
        )
        .1
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_1() {
        assert_eq!(day_03_1(">"), 2);
        assert_eq!(day_03_1("^>v<"), 4);
        assert_eq!(day_03_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_day_03_2() {
        assert_eq!(day_03_2("^v"), 3);
        assert_eq!(day_03_2("^>v<"), 3);
        assert_eq!(day_03_2("^v^v^v^v^v"), 11);
    }
}
