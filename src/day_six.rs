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
