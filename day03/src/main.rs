#![feature(iter_array_chunks)]

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> impl Iterator<Item = Coord> + '_ {
    input.chars().map(|c| match c {
        '^' => Coord { x: 0, y: 1 },
        'v' => Coord { x: 0, y: -1 },
        '>' => Coord { x: 1, y: 0 },
        '<' => Coord { x: -1, y: 0 },
        _ => unreachable!(),
    })
}

fn houses_visited(directions: impl IntoIterator<Item = Coord>) -> impl IntoIterator<Item = Coord> {
    let mut path = vec![];
    let mut pos = Coord::default();

    path.push(pos);
    for Coord { x, y } in directions {
        pos.x += x;
        pos.y += y;
        path.push(pos)
    }
    path
}

fn unique_houses(directions: impl IntoIterator<Item = Coord>) -> usize {
    houses_visited(directions)
        .into_iter()
        .collect::<HashSet<_>>()
        .len()
}

fn santa_and_robo_directions(
    directions: impl IntoIterator<Item = Coord>,
) -> (
    impl IntoIterator<Item = Coord>,
    impl IntoIterator<Item = Coord>,
) {
    directions
        .into_iter()
        .array_chunks::<2>()
        .map(|[a, b]| (a, b))
        .unzip::<_, _, Vec<_>, Vec<_>>()
}

fn unique_houses_with_robo_santa(directions: impl IntoIterator<Item = Coord>) -> usize {
    let (santa_directions, robo_santa_directions) = santa_and_robo_directions(directions);
    let santa_houses = houses_visited(santa_directions);
    let robo_santa_houses = houses_visited(robo_santa_directions);

    santa_houses
        .into_iter()
        .chain(robo_santa_houses)
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let input = include_str!("../input.txt");
    let directions = parse_input(input);

    println!("{}", unique_houses_with_robo_santa(directions));
}
