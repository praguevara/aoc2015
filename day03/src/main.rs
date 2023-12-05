use std::collections::HashSet;

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input.chars().map(|c| match c {
        '^' => (0, 1),
        'v' => (0, -1),
        '>' => (1, 0),
        '<' => (-1, 0),
        _ => unreachable!(),
    })
}

fn unique_houses(directions: impl IntoIterator<Item = (i32, i32)>) -> usize {
    let mut path = vec![];
    let mut pos = (0, 0);

    path.push(pos);
    for (x, y) in directions {
        pos.0 += x;
        pos.1 += y;
        path.push(pos)
    }

    path.iter().collect::<HashSet<_>>().len()
}

fn main() {
    let input = include_str!("../input.txt");
    let directions = parse_input(input);

    println!("{}", unique_houses(directions));
}
