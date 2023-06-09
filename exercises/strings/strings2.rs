// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
    println!();
    println!();

    // Hindi word, written in the Devanagari script
    let x = "नमस्ते";

    vector_values(x);
    println!();
    unicode_scalar_values(x);
    println!();
    with_ranges_be_careful(x);
    println!();
    iterating_over_strings(x);
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn vector_values(word: &str) {
    let byte_vec = word.as_bytes();
    println!("{:?}", byte_vec);
    // u8 vector values.
    println!("{} bytes long", byte_vec.len()) // 18
}

fn unicode_scalar_values(word: &str) {
    let char_type = word.chars();
    println!("{:?}", char_type);
    // ^ result looks a little different than in The Book; they note that the fourth
    // and sixth values are "diacritics" that don't make sense on their own -- and
    // those are the ones that look different... ex: '\u{94d}'

    /*
    It's important to remember that char represents a Unicode Scalar Value, and
    might not match your idea of what a 'character' is. Iteration over grapheme
    clusters may be what you actually want. This functionality is not provided
    by Rust's standard library, check crates.io instead.
    */
    let hm = word.chars();
    // ^ comment is from .chars() hover. grapheme clusters is what i was looking
    // for. they're the closest thing to what we would call letters.
}

fn with_ranges_be_careful(word: &str) {
    // let hello = &word[0];
    // ^ not possible. can do ranges:
    let hello = &word[0..6];
    println!("{:?}", hello)
    // be careful using ranges to create string slices. doing so can
    // crash your program
}

fn iterating_over_strings(word: &str) {
    // unicode scalar values
    for c in word.chars() {
        println!("{c}")
        // interestingly, the fourth and sixth chars look like they look in The
        // Book when iterating over them here.
    }

    // raw bytes
    for b in word.bytes() {
        println!("{b}")
    }
}
