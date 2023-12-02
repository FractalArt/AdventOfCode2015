//! # Advent of Code 2015 - Day 5
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2015/day/5).

fn is_nice_1(s: &str) -> bool {
    s.chars()
        .filter(|c| ['i', 'u', 'e', 'o', 'a'].contains(c))
        .count()
        >= 3
        && s.as_bytes().windows(2).filter(|w| w[0] == w[1]).count() > 0
        && !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"))
}

pub fn is_nice_2(s: &str) -> bool {
    s.as_bytes().windows(3).filter(|w| w[0] == w[2]).count() > 0
        && s.as_bytes()
            .windows(2)
            .map(|w| String::from_utf8(w.to_vec()).unwrap())
            .enumerate()
            .any(|(idx, w)| s[std::cmp::min(idx + 2, s.len() - 1)..].contains(&w))
}

/// The solution to task 1 of day 5.
pub fn day_05_1(data: &[String]) -> usize {
    data.iter().filter(|s| is_nice_1(s)).count()
}

/// The solution to task 2 of day 5.
pub fn day_05_2(data: &[String]) -> usize {
    data.iter().filter(|s| is_nice_2(s)).count()
}

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

    #[test]
    fn test_day_05_2() {
        let data = [
            "qjhvhtzxzqqjkmpb".to_string(),
            "xxyxx".to_string(),
            "uurcxstgmygtbstg".to_string(),
            "ieodomkazucvgmuy".to_string(),
        ];
        assert_eq!(day_05_2(&data), 2);
    }
}
