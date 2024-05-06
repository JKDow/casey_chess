

pub fn square_to_coords(square: &str) -> Option<(usize, usize)> {
    let square = square.bytes().collect::<Vec<u8>>();
    if square.len() != 2 { return None }
    let letter = square[0] - 97; 
    if letter > 7 { return None }
    let number = square[1] - 49; 
    if number > 7 { return None }
    return Some((letter as usize, number as usize))

}
