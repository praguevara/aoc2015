use std::collections::HashMap;

use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
struct Sue<'a> {
    number: usize,
    attributes: HashMap<&'a str, usize>,
}

fn parse_sue(input: &str) -> IResult<&str, Sue> {
    let (input, number) = nom::sequence::terminated(
        nom::sequence::preceded(
            nom::bytes::complete::tag("Sue "),
            nom::character::complete::digit1,
        ),
        nom::bytes::complete::tag(": "),
    )(input)?;
    let (input, attributes) = nom::multi::separated_list1(
        nom::bytes::complete::tag(", "),
        nom::sequence::pair(
            nom::bytes::complete::take_while1(|c: char| c.is_alphabetic()),
            nom::sequence::preceded(
                nom::bytes::complete::tag(": "),
                nom::character::complete::digit1,
            ),
        ),
    )(input)?;

    let attributes = attributes
        .into_iter()
        .map(|(s, n)| (s, n.parse().unwrap()))
        .collect::<HashMap<_, _>>();

    Ok((
        input,
        Sue {
            number: number.parse().unwrap(),
            attributes,
        },
    ))
}

fn find_target_sue<'a>(
    sues: &'a [Sue],
    search_attributes: &HashMap<&str, usize>,
) -> Option<&'a Sue<'a>> {
    sues.iter().find(|sue| {
        sue.attributes
            .iter()
            .all(|(k, n)| search_attributes.get(k) == Some(n))
    })
}

fn find_target_sue_2<'a>(
    sues: &'a [Sue],
    search_attributes: &HashMap<&str, usize>,
) -> Option<&'a Sue<'a>> {
    sues.iter().find(|sue| {
        sue.attributes.iter().all(|(&k, v)| match (k, v) {
            ("cats" | "trees", n) => n > search_attributes.get(k).unwrap(),
            ("pomeranians" | "goldfish", n) => n < search_attributes.get(k).unwrap(),
            (_, a) => search_attributes.get(k) == Some(a),
        })
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let sues = input
        .lines()
        .map(|line| parse_sue(line).unwrap().1)
        .collect::<Vec<_>>();

    let target_sue_attributes = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let target_sue = find_target_sue(&sues, &target_sue_attributes).unwrap();
    println!("{}", target_sue.number);

    let target_sue_2 = find_target_sue_2(&sues, &target_sue_attributes).unwrap();
    println!("{}", target_sue_2.number);
}
