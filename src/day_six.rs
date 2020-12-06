use std::collections::BTreeSet;

pub fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<BTreeSet<char>>()
                .len()
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|person| person.chars().collect::<BTreeSet<char>>())
                .fold_first(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}
