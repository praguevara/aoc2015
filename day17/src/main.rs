fn parse_input(input: &str) -> Vec<i32> {
    let mut input = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input.sort();
    input
}

fn find_combination(containers: &[i32], target: i32) -> usize {
    if target == 0 {
        return 1;
    }

    let mut num_combinations = 0;
    for (i, container) in containers.iter().enumerate() {
        if *container > target {
            break;
        }

        num_combinations += find_combination(&containers[(i + 1)..], target - container)
    }
    num_combinations
}

fn find_combination_minimum_containers(
    containers_left: &[i32],
    remaining_eggnog: i32,
    chosen_containers: Vec<i32>,
) -> Option<usize> {
    if remaining_eggnog == 0 {
        return Some(chosen_containers.len());
    }

    let mut best_solution = None;
    for (i, container) in containers_left.iter().enumerate() {
        if *container > remaining_eggnog {
            break;
        }

        let new_containers = {
            let mut new_containers = chosen_containers.clone();
            new_containers.push(*container);
            new_containers
        };

        let next_solution = find_combination_minimum_containers(
            &containers_left[(i + 1)..],
            remaining_eggnog - container,
            new_containers,
        );

        if let Some(next_solution) = next_solution {
            if let Some(best_solution) = &mut best_solution {
                if next_solution < *best_solution {
                    *best_solution = next_solution;
                }
            } else {
                best_solution = Some(next_solution);
            }
        }
    }
    best_solution
}

fn find_combination_n_containers(containers: &[i32], target: i32, remaining: usize) -> usize {
    match (remaining, target) {
        (0, 0) => 1,
        (0, _) | (_, 0) => 0,
        _ => {
            let mut num_combinations = 0;
            for (i, container) in containers.iter().enumerate() {
                if *container > target {
                    break;
                }

                num_combinations += find_combination_n_containers(
                    &containers[(i + 1)..],
                    target - container,
                    remaining - 1,
                )
            }
            num_combinations
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let containers = parse_input(input);

    let num_combinations = find_combination(&containers, 150);
    println!("{num_combinations}");

    let min_containers = find_combination_minimum_containers(&containers, 150, vec![]);
    let combinations_min_containers =
        find_combination_n_containers(&containers, 150, min_containers.unwrap());
    println!("{combinations_min_containers}");
}

#[test]
fn test() {
    let containers = [5, 5, 10, 15, 20];
    let num_combinations = find_combination(&containers, 25);
    assert_eq!(num_combinations, 4);
}

#[test]
fn test_2() {
    let containers = [5];
    let num_combinations = find_combination(&containers, 5);
    assert_eq!(num_combinations, 1);
}

#[test]
fn test_3() {
    let containers = [5, 5, 10];
    let num_combinations = find_combination(&containers, 10);
    assert_eq!(num_combinations, 2);
}

#[test]
fn test_4() {
    let containers = [5, 5];
    let num_combinations = find_combination(&containers, 5);
    assert_eq!(num_combinations, 2);
}

#[test]
fn test_min_containers() {
    let containers = [5, 5, 10];
    let min_containers = find_combination_minimum_containers(&containers, 10, vec![]);
    assert_eq!(min_containers, Some(1));
}
