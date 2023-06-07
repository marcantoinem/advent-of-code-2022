use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt").unwrap();
    let mut lines = lines.lines();
    let containers_txt: Vec<&str> = lines.by_ref().take_while(|x| x.contains('[')).collect();
    let len = containers_txt.last().unwrap().len();
    let mut containers = vec![vec![]; (len + 1) / 4];
    for x in containers_txt.iter().rev() {
        for i in (1..len).step_by(4) {
            if let Some(value) = x.get(i..i + 1) {
                if value != " " {
                    containers[(i - 1) / 4].push(value);
                }
            }
        }
    }
    lines.next();
    for instructions in lines {
        let instructions = instructions
            .chars()
            .filter(|x| x.is_numeric() || x.is_whitespace())
            .collect::<String>()
            .replace("  ", " ");
        let mut instructions = instructions.trim().split(' ');
        let number = instructions.next().unwrap().parse::<usize>().unwrap();
        let source = instructions.next().unwrap().parse::<usize>().unwrap() - 1;
        let destination = instructions.next().unwrap().parse::<usize>().unwrap() - 1;
        let len_container = containers[source].len();
        let to_move = containers[source]
            .drain(len_container - number..)
            .collect::<Vec<&str>>();
        containers[destination].extend(to_move);
    }
    println!(
        "{}",
        containers
            .iter()
            .filter_map(|vec| vec.last().copied())
            .collect::<String>()
    );
    // lines
    // .next()
    // .unwrap()
    // .split(' ')
    // .for_each(|x| println!("{}", x));
}
