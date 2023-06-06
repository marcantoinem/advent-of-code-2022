use itertools::{Chunk, Itertools};
use std::{fs, str::Lines};

fn priority(char: u32) -> u32 {
    if char >= 97 {
        char - 96
    } else {
        char - 38
    }
}

fn _backpack_common_two(backpack: &str) -> u32 {
    let backpack = backpack.as_bytes();
    let mut first = 0u128;
    let mut second = 0u128;
    for value in &backpack[0..backpack.len() / 2] {
        first |= 1 << *value as u128;
    }
    for value in &backpack[backpack.len() / 2..] {
        second |= 1 << *value as u128;
    }
    priority((first & second).ilog2())
}

const SIZE: usize = 3;

fn backpack_common(mut backpack: Chunk<Lines>) -> u32 {
    let mut data = [0u128; SIZE];
    for number in data.iter_mut() {
        for value in backpack.next().unwrap().as_bytes() {
            *number |= 1 << *value as u128;
        }
    }
    priority((data.iter().fold(u128::MAX, |acc, x| acc & x)).ilog2())
}

fn main() {
    println!(
        "{}",
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .chunks(3)
            .into_iter()
            .map(backpack_common)
            .sum::<u32>()
    );
}
