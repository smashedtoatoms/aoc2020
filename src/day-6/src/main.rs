use anyhow::Result;
use input_file::Input;

/// AOC6
fn main() -> Result<()> {
    let input = Input::from_args()?;
    let values = input.strings("\n").collect::<Vec<&str>>();
    println!(
        "Anyone Yes:   {}\nEveryone Yes: {}",
        get_anyone_yes(&values),
        get_everyone_yes(&values)
    );
    Ok(())
}

/// Returns the number of declaration form yes responses
fn get_anyone_yes(raw_values: &[&str]) -> usize {
    let mut yes_responses: usize = 0;
    for val in raw_values {
        let mut chars: Vec<char> = val.chars().filter(|c| c != &'\n').collect();
        chars.sort_unstable();
        chars.dedup();
        yes_responses += chars.len();
    }
    yes_responses
}

/// Returns the number of declaration forms where everyone said yes.
fn get_everyone_yes(raw_values: &[&str]) -> usize {
    let mut yes_responses: usize = 0;
    for val in raw_values {
        let group_values_length = val.lines().count();
        let mut chars = val.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
        chars.sort_unstable();
        chars.dedup();
        for c in chars {
            if val.matches(c).count() == group_values_length {
                yes_responses += 1
            }
        }
    }
    yes_responses
}

#[cfg(test)]
mod tests {
    use crate::get_anyone_yes;
    use crate::get_everyone_yes;

    #[test]
    fn anyone_yes() {
        assert_eq!(get_anyone_yes(&get_test_data()), 11);
    }

    #[test]
    fn everyone_yes() {
        assert_eq!(get_everyone_yes(&get_test_data()), 6);
    }

    /// Returns tests data.
    fn get_test_data() -> Vec<&'static str> {
        return vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"];
    }
}
