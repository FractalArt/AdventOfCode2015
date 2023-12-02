//! # Advent of Code 2015 - Day 4
//!
//! This module contains the solution of the [fourth day's challenges](https://adventofcode.com/2015/day/4).

/// The solution to task 1 of day 3.
pub fn day_04(data: &str, repeat: usize) -> usize {
    (1..)
        .map(|x| {
            (
                x,
                format!("{:x}", md5::compute(format!("{}{}", data, x).as_bytes())),
            )
        })
        .find(|(_, digest)| digest.starts_with(&"0".repeat(repeat)))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_1() {
        assert_eq!(day_04("abcdef", 5), 609043);
    }
}
