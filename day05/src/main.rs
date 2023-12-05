fn contains_at_least_three_vowels(string: &str) -> bool {
    string
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
        .nth(2)
        .is_some()
}

// This assumes all characters are 1 byte long
fn contains_at_least_one_letter_twice_in_a_row(string: &str) -> bool {
    string.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn does_not_contain_substrings(string: &str) -> bool {
    string
        .as_bytes()
        .windows(2)
        .all(|w| w != b"ab" && w != b"cd" && w != b"pq" && w != b"xy")
}

fn is_nice_str(string: &str) -> bool {
    contains_at_least_three_vowels(string)
        && contains_at_least_one_letter_twice_in_a_row(string)
        && does_not_contain_substrings(string)
}

fn main() {
    let input = include_str!("../input.txt");
    let strings = input.lines();
    let nice_strings = strings.filter(|s| is_nice_str(s));
    println!("{}", nice_strings.count())
}

#[test]
fn test_strings() {
    assert!(is_nice_str("ugknbfddgicrmopn"));
    assert!(is_nice_str("aaa"));
    assert!(!is_nice_str("jchzalrnumimnmhp"));
    assert!(!is_nice_str("haegwjzuvuyypxyu"));
    assert!(!is_nice_str("dvszwmarrgswjxmb"));
}
