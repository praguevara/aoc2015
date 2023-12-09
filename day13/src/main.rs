use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::map,
    IResult,
};

#[derive(Debug, Clone)]
struct Relationship {
    subject: String,
    object: String,
    happiness: i32,
}

fn parse_relationship(input: &str) -> IResult<&str, Relationship> {
    let (input, subject) = complete::alpha1(input)?;
    let (input, _) = tag(" would ")(input)?;
    let (input, multiplier) = alt((map(tag("gain "), |_| 1), map(tag("lose "), |_| -1)))(input)?;
    let (input, happiness_abs) = map(complete::digit1, |x: &str| x.parse::<i32>().unwrap())(input)?;
    let (input, _) = tag(" happiness units by sitting next to ")(input)?;
    let (input, object) = alpha1(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((
        input,
        Relationship {
            subject: subject.to_owned(),
            object: object.to_owned(),
            happiness: multiplier * happiness_abs,
        },
    ))
}

type RelationshipMatrix = HashMap<(String, String), i32>;

fn arrangement_happiness(
    relationship_matrix: &RelationshipMatrix,
    arrangement: &Vec<String>,
) -> i32 {
    let mut total_happiness = 0;
    for i in 0..arrangement.len() {
        let j = (i + 1) % arrangement.len();
        if i == j {
            continue;
        }

        let (a, b) = (&arrangement[i], &arrangement[j]);

        total_happiness += relationship_matrix.get(&(a.clone(), b.clone())).unwrap();
        total_happiness += relationship_matrix.get(&(b.clone(), a.clone())).unwrap();
    }
    total_happiness
}

fn explore_arrangements(
    relationship_matrix: &RelationshipMatrix,
    current_arrangement: Vec<String>,
    remaining_people: HashSet<String>,
    nodes_visited: &mut usize,
) -> (Vec<String>, i32) {
    *nodes_visited += 1;
    if remaining_people.is_empty() {
        let current_arrangement_happiness =
            arrangement_happiness(relationship_matrix, &current_arrangement);
        (current_arrangement, current_arrangement_happiness)
    } else {
        let next_arrangements = remaining_people
            .iter()
            .map(|p| {
                let mut next_arrangement = current_arrangement.clone();
                next_arrangement.push(p.clone());
                let mut next_remaining = remaining_people.clone();
                next_remaining.remove(p);
                explore_arrangements(
                    relationship_matrix,
                    next_arrangement,
                    next_remaining,
                    nodes_visited,
                )
            })
            .collect::<Vec<_>>();

        let mut best_arrangement_happiness =
            arrangement_happiness(relationship_matrix, &next_arrangements[0].0);
        let mut best_arrangement = &next_arrangements[0].0;
        for final_arrangement in next_arrangements.iter().skip(1) {
            let final_arrangement_happiness =
                arrangement_happiness(relationship_matrix, &final_arrangement.0);
            if final_arrangement_happiness > best_arrangement_happiness {
                best_arrangement_happiness = final_arrangement_happiness;
                best_arrangement = &final_arrangement.0;
            }
        }

        (best_arrangement.clone(), best_arrangement_happiness)
    }
}

fn find_best_arrangement(
    relationship_matrix: &RelationshipMatrix,
    people: &[String],
) -> (Vec<String>, i32) {
    let mut nodes_visited = 0;
    let solution = explore_arrangements(
        relationship_matrix,
        vec![],
        people.iter().cloned().collect(),
        &mut nodes_visited,
    );
    dbg!(nodes_visited);
    solution
}

fn main() {
    let input = include_str!("../input.txt");
    let relationships: Vec<_> = input
        .lines()
        .map(parse_relationship)
        .map(Result::unwrap)
        .map(|(_, r)| r)
        .collect();

    let relationship_matrix: RelationshipMatrix = relationships
        .iter()
        .cloned()
        .map(
            |Relationship {
                 subject,
                 object,
                 happiness,
             }| { ((subject, object), happiness) },
        )
        .collect();

    let remaining_people = relationships
        .iter()
        .map(
            |Relationship {
                 subject,
                 object: _,
                 happiness: _,
             }| subject,
        )
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let (best_arrangement, best_arrangement_happiness) =
        find_best_arrangement(&relationship_matrix, &remaining_people);

    println!("{:?}", best_arrangement);
    println!("{}", best_arrangement_happiness);

    let mut relationship_matrix_with_myself = relationship_matrix.clone();
    let myself_string = String::from("Myself");
    for people in remaining_people.iter() {
        relationship_matrix_with_myself.insert((people.clone(), myself_string.clone()), 0);
        relationship_matrix_with_myself.insert((myself_string.clone(), people.clone()), 0);
    }
    let mut remaining_people_with_myself = remaining_people.clone();
    remaining_people_with_myself.push(myself_string);

    let (best_arrangement_with_myself, best_arrangement_with_myself_happiness) =
        find_best_arrangement(
            &relationship_matrix_with_myself,
            &remaining_people_with_myself,
        );

    println!("{:?}", best_arrangement_with_myself);
    println!("{}", best_arrangement_with_myself_happiness);
}
