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

fn is_nice_str_first_part(string: &str) -> bool {
    contains_at_least_three_vowels(string)
        && contains_at_least_one_letter_twice_in_a_row(string)
        && does_not_contain_substrings(string)
}

fn contains_a_pair_that_repeats_without_overlap(string: &str) -> bool {
    let chars = string.as_bytes();
    for i in 0..chars.len() - 1 {
        let pair = [chars[i], chars[i + 1]];
        for j in i + 2..chars.len() - 1 {
            let possible_pair = [chars[j], chars[j + 1]];
            if pair == possible_pair {
                return true;
            }
        }
    }
    false
}

fn contains_one_repeating_letter_with_one_letter_in_between(string: &str) -> bool {
    string.as_bytes().windows(3).any(|w| w[0] == w[2])
}

fn is_nice_str_second_part(string: &str) -> bool {
    contains_a_pair_that_repeats_without_overlap(string)
        && contains_one_repeating_letter_with_one_letter_in_between(string)
}

fn main() {
    let input = include_str!("../input.txt");
    let strings = input.lines();
    let nice_strings = strings.filter(|s| is_nice_str_second_part(s));
    println!("{}", nice_strings.count())
}

#[test]
fn test_strings_first_part() {
    assert!(is_nice_str_first_part("ugknbfddgicrmopn"));
    assert!(is_nice_str_first_part("aaa"));
    assert!(!is_nice_str_first_part("jchzalrnumimnmhp"));
    assert!(!is_nice_str_first_part("haegwjzuvuyypxyu"));
    assert!(!is_nice_str_first_part("dvszwmarrgswjxmb"));
}

#[test]
fn test_strings_second_part() {
    assert!(is_nice_str_second_part("qjhvhtzxzqqjkmpb"));
    assert!(is_nice_str_second_part("xxyxx"));
    assert!(!is_nice_str_second_part("uurcxstgmygtbstg"));
    assert!(!is_nice_str_second_part("ieodomkazucvgmuy"));
}
