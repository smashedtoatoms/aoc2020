use input_file::Input;

fn main() {
    let input = Input::from_args().unwrap();
    let values = input.strings("\n").collect();
    println!(
        "Anyone Yes:   {}\nEveryone Yes: {}",
        get_anyone_yes(&values),
        get_everyone_yes(&values)
    );
}

fn get_anyone_yes(raw_values: &Vec<&str>) -> usize {
    return raw_values
        .iter()
        .fold(Vec::new(), |mut acc, val| {
            let mut chars = val.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
            chars.sort();
            chars.dedup();
            acc.push(chars.len());
            acc
        })
        .iter()
        .sum();
}

fn get_everyone_yes(raw_values: &Vec<&str>) -> usize {
    return raw_values
        .iter()
        .fold(Vec::new(), |mut acc, val| {
            let group_values_length = val.lines().collect::<Vec<&str>>().len();
            let mut chars = val.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
            chars.sort();
            chars.dedup();
            acc.push(
                chars
                    .iter()
                    .map(|c| {
                        if val.matches(*c).count() == group_values_length {
                            1
                        } else {
                            0
                        }
                    })
                    .sum(),
            );
            acc
        })
        .iter()
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::get_anyone_yes;
    use crate::get_everyone_yes;

    fn get_test_data() -> Vec<&'static str> {
        return vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"];
    }

    #[test]
    fn anyone_yes() {
        assert_eq!(get_anyone_yes(&get_test_data()), 11);
    }

    #[test]
    fn everyone_yes() {
        assert_eq!(get_everyone_yes(&get_test_data()), 6);
    }
}
