use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn file_to_vec<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<T>().unwrap())
        .collect()
}

pub fn file_to_set<T>(path: &str) -> BTreeSet<T>
where
    T: FromStr + Ord,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<T>().unwrap())
        .collect()
}

pub fn file_to_string(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
