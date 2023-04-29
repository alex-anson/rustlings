// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character: char = '🦊'; // Finish this line like the example! What's your favorite character?
                                     // Try a letter, try a number, try a special character, try a character
                                     // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // Compound types can group multiple values into one type. Rust has two
    // primitive compound types: tuples and arrays.

    // Tuple => general way of grouping x number of values with a variety of
    // types into one compound type. They have a fixed length.
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'x');
    // ^ considered a single compound element
    let x: () = ();

    // Arrays => every element must have the same type. Arrays have a fixed
    // length. (Vectors are more flexible - can grow and shrink. A vector is a
    // similar collection type provided by the standard library.)
}
