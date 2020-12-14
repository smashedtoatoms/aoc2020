use anyhow::{anyhow, Result};
use input_file::Input;

/// AOC3
fn main() -> Result<()> {
    Ok(())
}

/// Returns number of trees encountered in map.
fn find_trees(tree_map: &[&str]) -> Result<usize> {
    let mut x: usize = 3;
    let mut tree_count: usize = 0;

    for raw_row in tree_map {
        let row: Vec<char> = raw_row.chars().collect();
        let spot = row.get(x).ok_or_else(|| anyhow!("failed to pull column"))?;
        if spot == &'#' {
            tree_count += 1;
        }
        x += 3
    }

    Ok(tree_count)
}

#[cfg(test)]
mod tests {
    use crate::find_trees;

    #[test]
    fn test_condition_1() {
        assert_eq!(find_trees(&get_test_data()), 7);
    }

    #[test]
    fn test_condition_2() {
        assert_eq!(1, 1);
    }

    /// Returns test data.
    fn get_test_data() -> Vec<&str> {
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
