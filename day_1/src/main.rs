use itertools::Itertools;
use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() {
    let mut max = [0; 3];
    'outer_for: for result in BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .filter_map(|s| s.ok())
        .group_by(|s| s.len() != 0)
        .into_iter()
        .filter_map(|(key, s)| if key { Some(s) } else { None })
        .map(|s| {
            s.filter_map(|line| line.parse::<usize>().ok())
                .sum::<usize>()
        })
    {
        for i in 0..max.len() {
            if result > max[i] {
                for j in ((i + 1)..max.len()).rev() {
                    max[j] = max[j - 1];
                }
                max[i] = result;
                continue 'outer_for;
            }
        }
    }
    println!("{}", max.iter().sum::<usize>());
}
