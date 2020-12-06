use std::ops::RangeInclusive;

struct PartOnePolicyPasswordPair<'a> {
    password: &'a str,
    letter: char,
    allowed_occurrence_range: RangeInclusive<i32>,
}

impl PartOnePolicyPasswordPair<'_> {
    fn from_string(string: &str) -> PartOnePolicyPasswordPair {
        let mut parts = string.split(' ');
        let mut range_parts = parts.next().unwrap().split('-');
        let start = range_parts.next().unwrap().parse().unwrap();
        let end = range_parts.next().unwrap().parse().unwrap();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        PartOnePolicyPasswordPair {
            password,
            letter,
            allowed_occurrence_range: (start..=end),
        }
    }

    fn matches(&self) -> bool {
        let occurrences = self.password.chars().filter(|c| *c == self.letter).count() as i32;

        self.allowed_occurrence_range.contains(&occurrences)
    }
}

pub fn part_one(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|l| PartOnePolicyPasswordPair::from_string(l))
        .filter(PartOnePolicyPasswordPair::matches)
        .count()
}

struct PartTwoPolicyPasswordPair<'a> {
    password: &'a str,
    letter: char,
    num_1: usize,
    num_2: usize,
}

impl PartTwoPolicyPasswordPair<'_> {
    fn from_string(string: &str) -> PartTwoPolicyPasswordPair {
        let mut parts = string.split(' ');
        let mut num_parts = parts.next().unwrap().split('-');
        let num_1 = num_parts.next().unwrap().parse().unwrap();
        let num_2 = num_parts.next().unwrap().parse().unwrap();
        let letter = parts.next().unwrap().chars().next().unwrap();
        let password = parts.next().unwrap();

        PartTwoPolicyPasswordPair {
            password,
            letter,
            num_1,
            num_2,
        }
    }

    fn matches(&self) -> bool {
        let num_1_matches = self.password.chars().nth(self.num_1 - 1).unwrap() == self.letter;
        let num_2_matches = self.password.chars().nth(self.num_2 - 1).unwrap() == self.letter;

        num_1_matches != num_2_matches
    }
}

pub fn part_two(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|l| PartTwoPolicyPasswordPair::from_string(l))
        .filter(PartTwoPolicyPasswordPair::matches)
        .count()
}
