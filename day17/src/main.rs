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

fn main() {
    let input = include_str!("../input.txt");
    let containers = parse_input(input);

    dbg!(&containers);
    let num_combinations = find_combination(&containers, 150);
    println!("{num_combinations}");
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
