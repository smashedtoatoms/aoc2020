use anyhow::{anyhow, Result};
use input_file::Input;

/// Contains password and rules for password validity.
struct PasswordMetadata {
    pass: String,
    min: usize,
    max: usize,
    valid_char: char,
}

/// AOC2
fn main() -> Result<()> {
    let passwords = Input::from_args()?
        .strings("\n")
        .map(|v| parse_line(v))
        .collect::<Result<Vec<PasswordMetadata>>>()?;

    println!(
        "Valid Passwords (v1): {}\nValid Passwords (v2): {}",
        get_valid_pw_count_v1(&passwords),
        get_valid_pw_count_v2(&passwords)
    );
    Ok(())
}

/// Returns a count of passwords that conform to the v1 algorithm.
fn get_valid_pw_count_v1(passwords: &[PasswordMetadata]) -> usize {
    let mut password_count: usize = 0;
    for p in passwords {
        let char_count = p.pass.chars().filter(|c| c == &p.valid_char).count();
        if p.min <= char_count && p.max >= char_count {
            password_count += 1;
        }
    }
    password_count
}

/// Returns a count of the passwords that conform to the v2 algorithm.
fn get_valid_pw_count_v2(passwords: &[PasswordMetadata]) -> usize {
    let mut password_count: usize = 0;
    for p in passwords {
        let mut is_valid = false;
        for (i, letter) in p.pass.char_indices() {
            if (i == p.min - 1 || i == p.max - 1) && (letter == p.valid_char) {
                is_valid = !is_valid;
            }
        }
        if is_valid {
            password_count += 1;
        }
    }
    password_count
}

/// Returns PasswordMetadata parsed from a string...  There has to be a better way...
fn parse_line(line: &str) -> Result<PasswordMetadata> {
    let items: Vec<&str> = line.split(' ').collect();
    let min_max = items
        .get(0)
        .ok_or_else(|| anyhow!("failed to get min_max value from split"))?
        .split('-')
        .map(|x| x.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()?;
    let min = min_max
        .get(0)
        .ok_or_else(|| anyhow!("failed to parse min"))?
        .to_owned();
    let max = min_max
        .get(1)
        .ok_or_else(|| anyhow!("failed to parse min"))?
        .to_owned();
    let valid_char = items
        .get(1)
        .ok_or_else(|| anyhow!("failed to get valid_char from split"))?
        .replace(":", "")
        .chars()
        .collect::<Vec<char>>()
        .get(0)
        .ok_or_else(|| anyhow!("failed to get valid_char from vector"))?
        .to_owned();
    let pass = items
        .get(2)
        .ok_or_else(|| anyhow!("failed to parse password"))?
        .to_string();

    Ok(PasswordMetadata {
        pass,
        min,
        max,
        valid_char,
    })
}

#[cfg(test)]
mod tests {
    use crate::get_valid_pw_count_v1;
    use crate::get_valid_pw_count_v2;
    use crate::parse_line;
    use crate::PasswordMetadata;

    #[test]
    fn parses_line() {
        let parsed = parse_line("8-10 r: wat").unwrap();
        assert_eq!(parsed.min, 8);
        assert_eq!(parsed.max, 10);
        assert_eq!(parsed.pass, "wat");
        assert_eq!(parsed.valid_char, 'r');
    }

    #[test]
    fn gets_count_v1() {
        assert_eq!(get_valid_pw_count_v1(&get_passwords()), 2);
    }

    #[test]
    fn gets_count_v2() {
        assert_eq!(get_valid_pw_count_v2(&get_passwords()), 1);
    }

    /// Returns test data.
    fn get_passwords() -> Vec<PasswordMetadata> {
        vec![
            PasswordMetadata {
                pass: "abcde".to_owned(),
                min: 1,
                max: 3,
                valid_char: 'a',
            },
            PasswordMetadata {
                pass: "cdefg".to_owned(),
                min: 1,
                max: 3,
                valid_char: 'b',
            },
            PasswordMetadata {
                pass: "ccccccccc".to_owned(),
                min: 2,
                max: 9,
                valid_char: 'c',
            },
        ]
    }
}
