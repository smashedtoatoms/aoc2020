use input_file::Input;

struct PasswordMetadata {
    pass: String,
    min: usize,
    max: usize,
    valid_char: char,
}

fn main() {
    let input = Input::from_args().unwrap();
    let values = input.strings("\n");
    let passwords: Vec<PasswordMetadata> = values.map(|v| parse_line(v)).collect();

    println!(
        "Valid Passwords (v1): {}\nValid Passwords (v2): {}",
        get_valid_pw_count_v1(&passwords),
        get_valid_pw_count_v2(&passwords)
    );
}

/// Returns a count of passwords that conform to the v1 algorithm.
fn get_valid_pw_count_v1(passwords: &Vec<PasswordMetadata>) -> usize {
    passwords
        .iter()
        .filter(|password| {
            let char_count = password
                .pass
                .chars()
                .filter(|c| c == &password.valid_char)
                .count();
            password.min <= char_count && password.max >= char_count
        })
        .count()
}

/// Returns a count of the passwords that conform to the v2 algorithm.
fn get_valid_pw_count_v2(passwords: &Vec<PasswordMetadata>) -> usize {
    passwords
        .iter()
        .filter(|password| {
            password
                .pass
                .char_indices()
                .fold(false, |acc, (i, letter)| {
                    match i == password.min - 1 || i == password.max - 1 {
                        true => acc ^ (letter == password.valid_char),
                        _ => acc,
                    }
                })
        })
        .count()
}

/// Returns string parsed into PasswordMetadata...  This is a shit-show.
fn parse_line(line: &str) -> PasswordMetadata {
    let items: Vec<&str> = line.split(" ").collect();
    let min_max: Vec<usize> = items
        .get(0)
        .unwrap()
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let valid_char: char = items
        .get(1)
        .unwrap()
        .replace(":", "")
        .chars()
        .collect::<Vec<char>>()
        .get(0)
        .unwrap()
        .to_owned();
    let pass = items.get(2).unwrap().to_owned();

    PasswordMetadata {
        pass: pass.to_string(),
        min: min_max.get(0).unwrap().to_owned(),
        max: min_max.get(1).unwrap().to_owned(),
        valid_char: valid_char,
    }
}

#[cfg(test)]
mod tests {
    use crate::get_valid_pw_count_v1;
    use crate::get_valid_pw_count_v2;
    use crate::parse_line;
    use crate::PasswordMetadata;

    #[test]
    fn parses_line() {
        let parsed = parse_line("8-10 r: wat");
        assert_eq!(parsed.min, 8);
        assert_eq!(parsed.max, 10);
        assert_eq!(parsed.pass, "wat");
        assert_eq!(parsed.valid_char, 'r');
    }

    #[test]
    fn gets_count_v1() {
        let pws = vec![
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
        ];
        assert_eq!(get_valid_pw_count_v1(&pws), 2);
    }

    #[test]
    fn gets_count_v2() {
        let pws = vec![
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
        ];
        assert_eq!(get_valid_pw_count_v2(&pws), 1);
    }
}
