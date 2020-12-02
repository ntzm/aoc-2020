use crate::util::file_to_vec;
use std::ops::RangeInclusive;

struct PartOnePolicyPasswordPair<'a> {
    password: &'a str,
    letter: char,
    allowed_occurrence_range: RangeInclusive<i32>,
}

impl PartOnePolicyPasswordPair<'_> {
    fn from_string(string: &str) -> PartOnePolicyPasswordPair {
        let mut parts = string.split(' ');
        let range: Vec<i32> = parts
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        PartOnePolicyPasswordPair {
            password,
            letter,
            allowed_occurrence_range: (range[0]..=range[1]),
        }
    }

    fn matches(&self) -> bool {
        let occurrences = self.password.chars().filter(|c| *c == self.letter).count() as i32;

        self.allowed_occurrence_range.contains(&occurrences)
    }
}

pub fn part_one() -> usize {
    let lines: Vec<String> = file_to_vec("input/2.txt");

    lines
        .iter()
        .map(|e| PartOnePolicyPasswordPair::from_string(e))
        .filter(|p| p.matches())
        .count()
}

struct PartTwoPolicyPasswordPair<'a> {
    password: &'a str,
    letter: char,
    nums: Vec<usize>,
}

impl PartTwoPolicyPasswordPair<'_> {
    fn from_string(string: &str) -> PartTwoPolicyPasswordPair {
        let mut parts = string.split(' ');
        let nums = parts
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        PartTwoPolicyPasswordPair {
            password,
            letter,
            nums,
        }
    }

    fn matches(&self) -> bool {
        self.nums
            .iter()
            .filter(|n| self.password.chars().nth(*n - 1).unwrap() == self.letter)
            .count()
            == 1
    }
}

pub fn part_two() -> usize {
    let lines: Vec<String> = file_to_vec("input/2.txt");

    lines
        .iter()
        .map(|e| PartTwoPolicyPasswordPair::from_string(e))
        .filter(|p| p.matches())
        .count()
}
