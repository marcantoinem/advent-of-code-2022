#![feature(portable_simd)]

use std::{
    fs,
    simd::{u32x16, u8x16, u8x8, SimdPartialEq, SimdUint},
    time::Instant,
};

fn _problem1() {
    let text = fs::read_to_string("input.txt").unwrap();
    let code = text.as_bytes();
    let mut first_cmp = [0u8; 8];
    let mut second_cmp = [0, 0, 0, 0, 0, 0, 1, 1];
    let mut i = 0;
    loop {
        first_cmp[0..4].copy_from_slice(&code[i..i + 4]);
        first_cmp[4] = code[i];
        first_cmp[5..7].copy_from_slice(&code[i..i + 2]);
        second_cmp[0..3].copy_from_slice(&code[i + 1..i + 4]);
        second_cmp[3..5].copy_from_slice(&code[i + 2..i + 4]);
        second_cmp[5..7].copy_from_slice(&code[i + 2..i + 4]);
        let first_cmp_simd = u8x8::from_array(first_cmp);
        let second_cmp_simd = u8x8::from_array(second_cmp);
        if !first_cmp_simd.simd_eq(second_cmp_simd).any() {
            break;
        }
        i += 1;
    }
    let marker = i + 4;
    println!("{} {}", marker, &text[i..i + 4]);
}

fn main() {
    const ONES: u32x16 = u32x16::from_array([1; 16]);
    const TEXT: &str = include_str!("input.txt");
    let code = TEXT.as_bytes();
    let instant = Instant::now();
    let mut i = 0;
    loop {
        let mut extract = [0; 16];
        extract[0..14].copy_from_slice(&code[i..i + 14]);
        let character = u8x16::from_array(extract) - u8x16::from_array([97; 16]);
        let character = u32x16::from_array(character.as_array().map(|x| x as u32));
        if (ONES << character).reduce_or().count_ones() == 15 {
            break;
        }
        i += 1;
    }
    let marker = i + 14;
    println!(
        "{} {} computed in {} us",
        marker,
        &TEXT[i..i + 14],
        instant.elapsed().as_micros()
    );
}
