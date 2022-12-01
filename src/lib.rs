use std::{fs, path::Path};

/// Read a file with element seperated by a new line
/// Element are i32.
///
/// # Examples
///
/// ```
/// use presets::read_by_line_to_vec;
///
/// let result = read_by_line_to_vec("./one_int_per_line.txt");
/// assert_eq!(result, vec![1, 2, 3]);
/// ```
pub fn read_by_line_to_vec<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let lines: String = fs::read_to_string(filename).unwrap();
    let trimmed: &str = lines.trim();
    trimmed
        .split_terminator("\n")
        .map(|line| line.parse().unwrap())
        .collect()
}
