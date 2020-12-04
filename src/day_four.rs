use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
enum Height {
    Cm(u16),
    In(u16),
}

impl Height {
    fn from_string(input: &str) -> Option<Height> {
        let without_suffix = input
            .replace(|c: char| c.is_alphabetic(), "")
            .parse()
            .ok()?;

        Some(match input {
            h if h.ends_with("cm") => Height::Cm(without_suffix),
            h if h.ends_with("in") => Height::In(without_suffix),
            _ => return None,
        })
    }
}

#[derive(Debug)]
struct Passport<'a> {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
}

impl Passport<'_> {
    fn from_map<'a>(map: &'a HashMap<&str, &str>) -> Option<Passport<'a>> {
        Some(Passport {
            byr: map.get("byr")?.parse().ok()?,
            iyr: map.get("iyr")?.parse().ok()?,
            eyr: map.get("eyr")?.parse().ok()?,
            hgt: map.get("hgt")?,
            hcl: map.get("hcl")?,
            ecl: map.get("ecl")?,
            pid: map.get("pid")?,
        })
    }

    fn is_valid(&self) -> bool {
        let mut hcl_chars = self.hcl.chars();

        self.byr >= 1920
            && self.byr <= 2002
            && self.iyr >= 2010
            && self.iyr <= 2020
            && self.eyr >= 2020
            && self.eyr <= 2030
            && match Height::from_string(self.hgt) {
                Some(Height::Cm(h)) => h >= 150 && h <= 193,
                Some(Height::In(h)) => h >= 59 && h <= 76,
                None => false,
            }
            && hcl_chars.next().unwrap() == '#'
            && hcl_chars.all(|c| c.is_ascii_hexdigit())
            && self.hcl.len() == 7
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl)
            && self.pid.chars().all(|c| c.is_numeric())
            && self.pid.len() == 9
    }
}

pub fn part_one(input: &str) -> usize {
    let required_keys: BTreeSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    input
        .split("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|pair| pair.split(':').next().unwrap())
                .collect::<BTreeSet<_>>()
        })
        .filter(|keys| required_keys.difference(&keys).next().is_none())
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|pair| {
                    let mut parts = pair.split(':');
                    (parts.next().unwrap(), parts.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .filter_map(Passport::from_map)
        .filter(Passport::is_valid)
        .count()
}
