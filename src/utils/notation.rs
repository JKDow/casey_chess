
/// Converts a square in algrbratic chess notation to a pair of coordinates
/// # Description
/// This takes in algebratic chess coordinates such as 'e4' and converts them to a pair of coordinates
/// These coordinates are in the form of (x, y) where x is the column and y is the row
/// Returns None if the input is not a valid square
/// # Inputs/Outputs
/// - Input: square: &str - The square in algebratic chess notation
/// - Output: Option<(usize, usize)> - The coordinates of the square
/// # Example
/// ```Rust
/// let square = "e4";
/// let coords = square_to_coords(square);
/// assert_eq!(coords, Some((4, 3)));
/// ```
pub fn square_to_coords(square: &str) -> Option<(usize, usize)> {
    let square = square.bytes().collect::<Vec<u8>>();
    if square.len() != 2 { return None }
    let letter = square[0] - 97; 
    if letter > 7 { return None }
    let number = square[1] - 49; 
    if number > 7 { return None }
    return Some((letter as usize, number as usize))

}
