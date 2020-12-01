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
