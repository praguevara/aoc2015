fn count_chars_literals_values(input: &str) -> (usize, usize) {
    let mut literals = 0;
    let mut values = 0;

    let mut escaping = false;
    let mut escaping_hex_remaining = 0;

    for c in input.chars() {
        match c {
            '\\' => {
                if escaping {
                    // "\\"
                    escaping = false;
                    values += 1;
                } else {
                    // "\"
                    escaping = true;
                }
            }
            'x' => {
                if escaping {
                    if escaping_hex_remaining == 0 {
                        // "\x"
                        escaping_hex_remaining = 2;
                    } else {
                        // "\xx"
                        escaping_hex_remaining -= 1;
                        if escaping_hex_remaining == 0 {
                            // "\xAx"
                            escaping = false;
                            values += 1;
                        }
                    }
                } else {
                    values += 1;
                }
            }
            '\"' => {
                if escaping {
                    if escaping_hex_remaining > 0 {
                        escaping_hex_remaining -= 1;
                        if escaping_hex_remaining == 0 {
                            // "\xA\""
                            escaping = false;
                            values += 1;
                        }
                    } else {
                        // "\\"
                        escaping = false;
                        values += 1;
                    }
                }
            }
            _ => {
                if escaping && escaping_hex_remaining > 0 {
                    // "\xA"
                    escaping_hex_remaining -= 1;
                }
                if escaping_hex_remaining == 0 {
                    // "\xAA"
                    escaping = false;
                    values += 1;
                }
            }
        }
        literals += 1;
    }

    (literals, values)
}

fn length_encoded_string(input: &str) -> usize {
    dbg!(input);
    input
        .chars()
        .map(|c| match c {
            '\"' => 2,
            '\\' => 2,
            _ => 1,
        })
        .sum::<usize>()
        + 2
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.lines();

    let encoded: usize = lines.clone().map(length_encoded_string).sum();
    let (literals, _values) = lines
        .map(|line| {
            let (a, b) = count_chars_literals_values(line);
            (a, b)
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();
    println!("{}", encoded - literals);
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn sample_input() -> Vec<&'static str> {
        vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""]
    }

    #[test]
    fn test() {
        assert_eq!(count_chars_literals_values("xxx"), (3, 3));
    }

    #[test]
    fn test_count_chars_literals_values() {
        let sample_input = sample_input();
        let expected = [(2usize, 0usize), (5, 3), (10, 7), (6, 1)];
        let actual: Vec<_> = sample_input
            .iter()
            .map(|s| count_chars_literals_values(s))
            .collect();

        expected
            .iter()
            .zip(actual)
            .zip(sample_input)
            .for_each(|((e, a), i)| assert_eq!(*e, a, "{}", i));
    }

    #[test]
    fn test_length_encoded_string() {
        let sample_input = sample_input();
        let expected_strings = [
            "\"\\\"\\\"\"",
            "\"\\\"abc\\\"\"",
            "\"\\\"aaa\\\\\\\"aaa\\\"\"",
            "\"\\\"\\\\x27\\\"\"",
        ];
        let expected_lengths = expected_strings.map(str::len);
        let lengths = sample_input.iter().map(|s| length_encoded_string(s));
        expected_lengths
            .iter()
            .zip(lengths)
            .zip(sample_input.iter().zip(expected_strings))
            .for_each(|((e, a), s)| assert_eq!(*e, a, "{:?}", s));
    }
}
