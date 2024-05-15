
#[test]
fn square_to_coords_test() {
    let square = "e4";
    let coords = crate::utils::notation::square_to_coords(square);
    assert_eq!(coords, Some((4, 3)));
    
    let square = "a1";
    let coords = crate::utils::notation::square_to_coords(square);
    assert_eq!(coords, Some((0, 0)));

    let square = "h8";
    let coords = crate::utils::notation::square_to_coords(square);
    assert_eq!(coords, Some((7, 7)));
}
