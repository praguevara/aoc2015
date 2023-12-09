use itertools::Itertools;

fn increment_password(password: &mut [u8]) {
    for c in password.iter_mut().rev() {
        *c = ((*c - b'a' + 1) % (b'z' - b'a' + 1)) + b'a';
        if *c != b'a' {
            break;
        }
    }
}

fn contains_three_straight_increasing_letters(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[1] as i32 - w[0] as i32 == 1 && w[2] as i32 - w[1] as i32 == 1)
}

fn does_not_contain_forbidden_letters(password: &[u8]) -> bool {
    !password.iter().any(|c| [b'i', b'o', b'l'].contains(c))
}

fn different_non_overlapping_paris(password: &[u8]) -> Vec<u8> {
    password
        .iter()
        .group_by(|c| *c)
        .into_iter()
        .filter_map(|(c, l)| if l.count() == 2 { Some(*c) } else { None })
        .collect()
}

fn contains_two_different_non_overlapping_pairs(password: &[u8]) -> bool {
    different_non_overlapping_paris(password)
        .iter()
        .unique()
        .count()
        >= 2
}

fn generate_new_password(old_password: &mut [u8]) -> Vec<u8> {
    increment_password(old_password);
    let tentative_password = old_password;
    while !contains_three_straight_increasing_letters(tentative_password)
        || !does_not_contain_forbidden_letters(tentative_password)
        || !contains_two_different_non_overlapping_pairs(tentative_password)
    {
        increment_password(tentative_password);
    }

    tentative_password.into()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut password = input.trim().as_bytes().to_owned();
    let mut new_password = generate_new_password(&mut password);
    println!("{}", String::from_utf8_lossy(&new_password));

    let new_password = generate_new_password(&mut new_password);
    println!("{}", String::from_utf8_lossy(&new_password));
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use super::*;

    #[test]
    fn test_increment_password() {
        let mut password = b"abc".to_vec();
        increment_password(&mut password);
        assert_eq!(password, b"abd");

        let mut password = b"xz".to_vec();
        increment_password(&mut password);
        assert_eq!(password, b"ya");
    }

    #[test]
    fn test_new_passwords() {
        let passwords = [
            "abcdefgh".as_bytes().to_owned(),
            "ghijklmn".as_bytes().to_owned(),
        ];
        let next_passwords = [
            "abcdffaa".as_bytes().to_owned(),
            "ghjaabcc".as_bytes().to_owned(),
        ];

        for (p, n) in passwords.iter().zip(next_passwords) {
            let actual = generate_new_password(p.clone().deref_mut());
            assert_eq!(
                String::from_utf8_lossy(&actual),
                String::from_utf8_lossy(&n),
            );
        }
    }
}
