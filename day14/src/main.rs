use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    IResult,
};

#[derive(Debug)]
struct Raindeer {
    name: String,
    speed: usize,
    speed_seconds: usize,
    rest: usize,
}

fn parse_raindeer(input: &str) -> IResult<&str, Raindeer> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" can fly ")(input)?;
    let (input, speed) = map(digit1, |x: &str| x.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(" km/s for ")(input)?;
    let (input, speed_seconds) = map(digit1, |x: &str| x.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(" seconds, but then must rest for ")(input)?;
    let (input, rest) = map(digit1, |x: &str| x.parse::<usize>().unwrap())(input)?;
    let (input, _) = tag(" seconds.")(input)?;

    Ok((
        input,
        Raindeer {
            name: name.to_owned(),
            speed,
            speed_seconds,
            rest,
        },
    ))
}

fn race_raindeer_for_seconds(raindeer: &Raindeer, seconds: usize) -> usize {
    let Raindeer {
        name: _,
        speed,
        speed_seconds,
        rest,
    } = raindeer;

    if seconds == 0 {
        return 0;
    }

    let run_seconds = speed_seconds.min(&seconds);
    let run_distance = speed * run_seconds;
    let next_race_seconds = run_seconds + rest;

    run_distance + race_raindeer_for_seconds(raindeer, seconds - next_race_seconds.min(seconds))
}

#[test]
fn test_race() {
    let raindeer = Raindeer {
        name: "Comet".into(),
        speed: 14,
        speed_seconds: 10,
        rest: 127,
    };

    assert_eq!(race_raindeer_for_seconds(&raindeer, 0), 0);
    assert_eq!(race_raindeer_for_seconds(&raindeer, 1), 14);
    assert_eq!(race_raindeer_for_seconds(&raindeer, 10), 140);
    assert_eq!(race_raindeer_for_seconds(&raindeer, 1000), 1120);
}

fn race_by_seconds(
    raindeer: &Raindeer,
    remaining_running_seconds: usize,
    remaining_rest_seconds: usize,
) -> (usize, usize, usize) {
    if remaining_running_seconds > 0 {
        (
            raindeer.speed,
            remaining_running_seconds - 1,
            remaining_rest_seconds,
        )
    } else if remaining_rest_seconds > 0 {
        (0, 0, remaining_rest_seconds - 1)
    } else {
        (raindeer.speed, raindeer.speed_seconds - 1, raindeer.rest)
    }
}

#[test]
fn test_race_by_seconds() {
    let raindeer = Raindeer {
        name: "Test".into(),
        speed: 2,
        speed_seconds: 2,
        rest: 2,
    };

    assert_eq!(race_by_seconds(&raindeer, 0, 0), (2, 1, 2));
    assert_eq!(race_by_seconds(&raindeer, 1, 2), (2, 0, 2));
    assert_eq!(race_by_seconds(&raindeer, 0, 2), (0, 0, 1));
    assert_eq!(race_by_seconds(&raindeer, 0, 1), (0, 0, 0));
}

fn main() {
    let input = include_str!("../input.txt");
    let raindeers = input
        .lines()
        .map(parse_raindeer)
        .map(Result::unwrap)
        .map(|(_, r)| r)
        .collect::<Vec<_>>();

    let race_for_seconds = 2503;

    let winner_distance = raindeers
        .iter()
        .map(|r| race_raindeer_for_seconds(r, race_for_seconds))
        .max()
        .unwrap();

    println!("{}", winner_distance);

    let res = raindeers
        .iter()
        .map(|r| {
            (0..=race_for_seconds)
                .scan((0, 0, 0), |(dist, rem, rest), _| {
                    let (d, new_rem_running, new_rem_rest) = race_by_seconds(r, *rem, *rest);
                    *dist += d;
                    *rem = new_rem_running;
                    *rest = new_rem_rest;
                    Some(*dist)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut wins = vec![0; raindeers.len()];
    for second in 0..=race_for_seconds {
        let current_second = res.iter().map(|r| r[second]);
        let winner_distance = current_second.clone().max().unwrap();
        current_second
            .enumerate()
            .filter(|(_, d)| *d == winner_distance)
            .for_each(|(i, _)| wins[i] += 1);
    }

    let points_of_winner = wins.iter().max().unwrap();
    println!("{}", points_of_winner);
}
