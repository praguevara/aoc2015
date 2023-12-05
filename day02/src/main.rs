fn parse_input(input_str: &str) -> Vec<[i32; 3]> {
    let lines = input_str.lines();
    lines
        .map(|line| {
            line.split('x')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!())
        })
        .collect()
}

fn dimensions_to_paper(mut dimensions: [i32; 3]) -> i32 {
    dimensions.sort();

    let exact_paper = 2
        * (dimensions[0] * dimensions[1]
            + dimensions[0] * dimensions[2]
            + dimensions[1] * dimensions[2]);
    let extra_paper = dimensions[0] * dimensions[1];
    exact_paper + extra_paper
}

#[test]
fn test_dimensions_to_paper() {
    let dims = [2, 3, 4];
    assert_eq!(dimensions_to_paper(dims), 58);
}

fn dimensions_to_ribbon(mut dimensions: [i32; 3]) -> i32 {
    dimensions.sort();

    let perimeter = 2 * (dimensions[0] + dimensions[1]);
    let volume = dimensions[0] * dimensions[1] * dimensions[2];
    perimeter + volume
}

#[test]
fn test_dimensions_to_ribbon() {
    let dims = [2, 3, 4];
    assert_eq!(dimensions_to_ribbon(dims), 34);
}

#[test]
fn test_parse_input() {
    let input_str = "20x29x30\r\n23x11x5\r\n";
    let expected = vec![[20, 29, 30], [23, 11, 5]];
    assert_eq!(parse_input(input_str), expected);
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);

    println!(
        "{}",
        input
            .iter()
            .map(|ds| dimensions_to_ribbon(*ds))
            .sum::<i32>()
    )
}
