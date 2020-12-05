use input_file::get_numbers;

/// AOC1
fn main() {
    let numbers = get_numbers();
    println!(
        "Pair Product:  {:?}\nTriad Product: {:?}\n",
        get_2020_pair_product(&numbers),
        get_2020_triad_product(&numbers)
    );
}

/// Returns the product of the pair of values in the provided list that add up
/// to 2020.
fn get_2020_pair_product(numbers: &Vec<i32>) -> i32 {
    return numbers
        .iter()
        .enumerate()
        .find_map(|(i, &number)| {
            match numbers
                .iter()
                .enumerate()
                .find(|(i2, &number2)| &i != i2 && (number + number2 == 2020))
            {
                Some((_, last)) => Some(number * last),
                None => None,
            }
        })
        .unwrap();
}

/// Returns the product of the triad of values in the provided list that up to
/// 2020.
fn get_2020_triad_product(numbers: &Vec<i32>) -> i32 {
    return numbers
        .iter()
        .enumerate()
        .find_map(|(i, &number)| {
            numbers.iter().enumerate().find_map(|(i2, &number2)| {
                match numbers.iter().enumerate().find(|(i3, &number3)| {
                    i != i2 && &i != i3 && &i2 != i3 && (number + number2 + number3 == 2020)
                }) {
                    Some((_, last)) => Some(number * number2 * last),
                    None => None,
                }
            })
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::get_2020_pair_product;
    use crate::get_2020_triad_product;

    #[test]
    fn gets_2020_pair_product() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(get_2020_pair_product(&numbers), 514579);
    }

    #[test]
    fn gets_2020_triad_product() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(get_2020_triad_product(&numbers), 241861950);
    }
}
