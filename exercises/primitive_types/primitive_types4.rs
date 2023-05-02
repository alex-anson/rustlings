// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // tried &a[1..3] first, it's obviously not inclusive
    // let nice_slice = &a[1..4];
    // COOOOOL
    let nice_slice = &a[1..a.len() - 1];
    // ^ this slice works the same way string slices work, by storing a
    // reference to the first element and a length.

    assert_eq!([2, 3, 4], nice_slice)
}
