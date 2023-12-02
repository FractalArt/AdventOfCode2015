//! # Advent of Code 2015 - Day 5
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2015/day/5).

fn is_nice(s: &str) -> bool {
    if s.chars()
        .filter(|c| ['i', 'u', 'e', 'o', 'a'].contains(c))
        .count()
        < 3
    {
        return false;
    }

    if s.as_bytes().windows(2).filter(|w| w[0] == w[1]).count() < 1 {
        return false;
    }

    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        return false;
    }

    true
}

/// The solution to task 1 of day 5.
pub fn day_05_1(data: &[String]) -> usize {
    data.iter().filter(|s| is_nice(s)).count()
}

/// The solution to task 2 of day 5.
//pub fn day_05_2(data: &[String]) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_1() {
        let data = [
            "ugknbfddgicrmopn".to_string(),
            "aaa".to_string(),
            "jchzalrnumimnmhp".to_string(),
            "haegwjzuvuyypxyu".to_string(),
            "dvszwmarrgswjxmb".to_string(),
        ];
        assert_eq!(day_05_1(&data), 2);
    }

    //#[test]
    //fn test_day_05_2() {
    //}
}
