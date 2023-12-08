use itertools::Itertools;

fn transform_sequence(input: &[usize]) -> Vec<usize> {
    input
        .iter()
        .group_by(|x| *x)
        .into_iter()
        .flat_map(|(k, g)| [g.count(), *k])
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut sequence = input
        .trim()
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap() as usize)
        .collect::<Vec<_>>();

    for _ in 0..40 {
        sequence = transform_sequence(&sequence);
    }
    dbg!(sequence.len());

    for _ in 40..50 {
        sequence = transform_sequence(&sequence);
    }
    dbg!(sequence.len());
}
