use std::fs;

fn value(text: &str) -> (u16, u16) {
    let mut values = text.split('-');
    (
        values.next().unwrap().parse::<u16>().unwrap(),
        values.next().unwrap().parse::<u16>().unwrap(),
    )
}

fn contains_other(text: &&str) -> bool {
    let mut parts = text.split(',');
    let (a, b) = value(parts.next().unwrap());
    let (c, d) = value(parts.next().unwrap());
    !((b < c) || (a > d))
}

fn main() {
    let result = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(contains_other)
        .count();
    println!("{}", result);
}
