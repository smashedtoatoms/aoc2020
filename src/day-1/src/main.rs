use anyhow::{anyhow, Result};
use input_file::Input;

/// AOC1
fn main() -> Result<()> {
    let numbers: Vec<u32> = Input::from_args()?.numbers("\n").collect();
    println!(
        "Pair Product:  {:?}\nTriad Product: {:?}\n",
        get_2020_pair_product(&numbers)?,
        get_2020_triad_product(&numbers)?
    );
    Ok(())
}

/// Returns the product of the pair of values in the provided list that add up
/// to 2020.
fn get_2020_pair_product(numbers: &[u32]) -> Result<u32> {
    for (i, a) in numbers.iter().enumerate() {
        for b in &numbers[i..] {
            if a + b == 2020 {
                return Ok(a * b);
            }
        }
    }

    Err(anyhow!("failed to find the pair"))
}

/// Returns the product of the triad of values in the provided list that up to
/// 2020.
fn get_2020_triad_product(numbers: &[u32]) -> Result<u32> {
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers[i..].iter().enumerate() {
            for c in &numbers[j..] {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }

    Err(anyhow!("failed to find the triad"))
}

#[cfg(test)]
mod tests {
    use crate::get_2020_pair_product;
    use crate::get_2020_triad_product;

    #[test]
    fn gets_2020_pair_product() {
        assert_eq!(get_2020_pair_product(&get_numbers()).unwrap(), 514579);
    }

    #[test]
    fn gets_2020_triad_product() {
        assert_eq!(get_2020_triad_product(&get_numbers()).unwrap(), 241861950);
    }

    /// Returns test data.
    fn get_numbers() -> Vec<u32> {
        vec![1721, 979, 366, 299, 675, 1456]
    }
}
