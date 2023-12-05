fn part_1(input_str: &str) -> i32 {
    input_str
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

fn part_2(input_str: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input_str.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor == -1 {
            return i + 1;
        }
    }
    unreachable!()
}

fn main() {
    let input_str = include_str!("../input.txt").trim();
    println!("{}", part_2(input_str));
}
