use anyhow::{anyhow, Result};
use input_file::Input;

/// AOC3
fn main() -> Result<()> {
    let input = Input::from_args()?;
    let tile: Vec<&str> = input.strings("\n").collect();
    println!(
        "Tree Count:         {:?}\nTree Count Product: {:?}\n",
        find_trees(&tile, 3, 1)?,
        find_multi_slope_product(&tile)?
    );
    Ok(())
}

/// Returns number of trees encountered in map.
fn find_trees(tile: &[&str], x_size: usize, y_size: usize) -> Result<usize> {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count: usize = 0;
    let tile_width = (tile
        .get(0)
        .ok_or_else(|| anyhow!("failed to pull first column"))?)
    .len();

    while y < tile.len() {
        let row: Vec<char> = tile
            .get(y)
            .ok_or_else(|| anyhow!("failed to pull row"))?
            .chars()
            .collect();
        let spot = row.get(x).ok_or_else(|| anyhow!("failed to pull column"))?;
        if spot == &'#' {
            tree_count += 1;
        }
        if x + x_size >= tile_width {
            x = (x + x_size) - tile_width;
        } else {
            x += x_size;
        }
        y += y_size;
    }

    Ok(tree_count)
}

/// Returns the product of the trees encountered using the various specified
/// routes.
fn find_multi_slope_product(tree_map: &[&str]) -> Result<usize> {
    Ok(find_trees(tree_map, 1, 1)?
        * find_trees(tree_map, 3, 1)?
        * find_trees(tree_map, 5, 1)?
        * find_trees(tree_map, 7, 1)?
        * find_trees(tree_map, 1, 2)?)
}

#[cfg(test)]
mod tests {
    use crate::{find_multi_slope_product, find_trees};

    #[test]
    fn test_condition_1() {
        assert_eq!(find_trees(&get_test_data(), 3, 1).unwrap(), 7);
    }

    #[test]
    fn test_condition_2() {
        assert_eq!(find_multi_slope_product(&get_test_data()).unwrap(), 336);
    }

    /// Returns test data.
    fn get_test_data<'a>() -> Vec<&'a str> {
        vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
    }
}
