use std::collections::{BTreeSet, HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Distance {
    from: String,
    to: String,
    distance: usize,
}

fn parse_distance(input: &str) -> IResult<&str, Distance> {
    let (input, from) = alpha1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = alpha1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, distance) = digit1(input)?;
    Ok((
        input,
        Distance {
            from: from.to_string(),
            to: to.to_string(),
            distance: distance.parse().unwrap(),
        },
    ))
}

fn parse_input(input: &str) -> Vec<Distance> {
    input
        .lines()
        .map(|line| parse_distance(line).unwrap().1)
        .collect()
}

type Route = Vec<String>;

#[derive(Debug)]
struct Distances {
    locations: Vec<String>,
    map: HashMap<(String, String), usize>,
}

impl Distances {
    fn compute_key(distance: (&String, &String)) -> (String, String) {
        let (from, to) = distance;
        (
            std::cmp::min(from, to).clone(),
            std::cmp::max(from, to).clone(),
        )
    }

    pub fn get(&self, distance: (&String, &String)) -> Option<usize> {
        let key = Distances::compute_key(distance);
        self.map.get(&key).copied()
    }

    pub fn new(distances: Vec<Distance>) -> Self {
        let locations = distances
            .iter()
            .flat_map(
                |Distance {
                     from,
                     to,
                     distance: _,
                 }| [from, to],
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        distances
            .into_iter()
            .for_each(|Distance { from, to, distance }| {
                let key = Distances::compute_key((&from, &to));
                map.insert(key, distance);
            });

        Distances { locations, map }
    }

    pub fn route_length(&self, route: &Route) -> usize {
        route
            .iter()
            .zip(route.iter().skip(1))
            .map(|(l, r)| self.get((l, r)).unwrap())
            .sum()
    }

    pub fn shortest_route(self) -> (Route, usize) {
        let mut best_route: Route = self.locations.clone().into_iter().collect();
        let mut best_route_length = self.route_length(&best_route);
        let mut stack: Vec<(usize, Route, HashSet<String>)> =
            vec![(0, vec![], self.locations.iter().cloned().collect())];

        let mut steps = 0;

        while let Some((current_route_length, current_route, remaining)) = stack.pop() {
            steps += 1;

            // Upper bound
            if current_route_length > best_route_length {
                continue;
            }

            if remaining.is_empty() {
                if current_route_length < best_route_length {
                    best_route = current_route;
                    best_route_length = current_route_length;
                }
            } else {
                for remaining_location in remaining.iter() {
                    let mut new_route = current_route.clone();
                    let last_location = new_route.last();

                    let mut new_remaining = remaining.clone();
                    new_remaining.remove(remaining_location);

                    let new_route_length = current_route_length
                        + if let Some(last_location) = last_location {
                            self.get((last_location, remaining_location)).unwrap()
                        } else {
                            0
                        };

                    new_route.push(remaining_location.clone());
                    stack.push((new_route_length, new_route, new_remaining));
                }
            }
        }

        // dbg!(steps);

        (best_route, best_route_length)
    }

    pub fn longest_route(self) -> (Route, usize) {
        let mut best_route: Route = self.locations.clone().into_iter().collect();
        let mut best_route_length = self.route_length(&best_route);
        let mut stack: Vec<(usize, Route, HashSet<String>)> =
            vec![(0, vec![], self.locations.iter().cloned().collect())];

        let mut steps = 0;

        while let Some((current_route_length, current_route, remaining)) = stack.pop() {
            steps += 1;

            if remaining.is_empty() {
                if current_route_length > best_route_length {
                    best_route = current_route;
                    best_route_length = current_route_length;
                }
            } else {
                for remaining_location in remaining.iter() {
                    let mut new_route = current_route.clone();
                    let last_location = new_route.last();

                    let mut new_remaining = remaining.clone();
                    new_remaining.remove(remaining_location);

                    let new_route_length = current_route_length
                        + if let Some(last_location) = last_location {
                            self.get((last_location, remaining_location)).unwrap()
                        } else {
                            0
                        };

                    new_route.push(remaining_location.clone());
                    stack.push((new_route_length, new_route, new_remaining));
                }
            }
        }

        // dbg!(steps);

        (best_route, best_route_length)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let distances_vec = parse_input(input);
    let distances = Distances::new(distances_vec);
    let longest_route = distances.longest_route();
    println!("{}", longest_route.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_distance() {
        let input = "London to Dublin = 464\r\n";
        let expected = Distance {
            from: "London".to_string(),
            to: "Dublin".to_string(),
            distance: 464,
        };
        assert_eq!(parse_distance(input).unwrap().1, expected);
    }

    #[test]
    fn test_shortest_route() {
        let input = include_str!("../sample.txt");
        let distances_vec = parse_input(input);
        let distances = Distances::new(distances_vec);
        dbg!(distances.shortest_route());
    }
}
