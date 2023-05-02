// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // let {name, age} = cat; ...nope
    // let [name, age] = cat; ...nope
    // they call this destructuring 🍫   yay for js similarities
    let (name, age) = cat; // ...bingo

    println!("{} is {} years old.", name, age);

    println!("");

    let s = "aaahhhh, so that's ⌄ how to create the &String type...";
    let result = first_word(&String::from(s));
    println!("{result}");

    println!("");
    string_slices();

    println!("");
    let binding = String::from(s);
    let result = first_word_proper(&binding);
    println!("proper: {result}");

    println!("");
    let my_string = String::from("hello world");

    // `first_word_proper_signature` works on slices of `String`s, whether
    // partial or whole
    let word = first_word_proper_signature(&my_string[0..6]);
    println!("{word}");
    let word = first_word_proper_signature(&my_string[..]);
    println!("{word}");
    // `first_word_proper_signature` also works on references to `String`s,
    // which are equivalent to whole slices of `String`s
    let word = first_word_proper_signature(&my_string);
    println!("{word}");

    let my_string_literal = "hello world";

    // `first_word_proper_signature` works on slices of string literals,
    // whether partial or whole
    let word = first_word_proper_signature(&my_string_literal[0..6]);
    println!("{word}");
    let word = first_word_proper_signature(&my_string_literal[..]);
    println!("{word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_proper_signature(my_string_literal);
    println!("{word}");
}

/*
Slices let you reference a contiguous sequence of elements in a collection
rather than the whole collection. A slice is a kind of reference, so it does
not have ownership.
*/

fn first_word(s: &String) -> usize {
    // convert string to an array of bytes
    let bytes = s.as_bytes();

    // .iter() is a method that returns each element in a collection
    // .enumerate() wraps the result of .iter() & returns each element as
    // part of a tuple.
    // the first element of the tuple is the index (`i`), & the second
    // (`item`) is a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {
        // ^ destructure the tuple
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// ^ the problem is that this is not tied to the input string at all. could
// clear that string out, and then the result from this ƒn would be totally invalid.

// enter: string slices
// a string slice is a reference to part of a `String`
fn string_slices() {
    let s = String::from("hello world");
    // 5 is not inclusive
    let hello = &s[0..5];
    // if starting at index 0, can drop the 0.
    let hello_equivalent = &s[..5];
    // variable `world` is a slice containing a POINTER to the byte at index 6
    // of variable `s`, with a length value of 5.
    let world = &s[6..11];
    // can drop trailing index as well
    let world_equivalent = &s[6..];
    println!("first word: {}, second word: {}", hello, world);
    println!("world's length: {}", world.len());

    // can also take a slice of an entire string:
    let whole_thing = &s[..];
    println!("{}", whole_thing);
}

// WOOOOWWHHOOO i did it without looking!!
fn first_word_proper(s: &String) -> &str {
    // convert string to an array of bytes
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // ^ destructure the tuple
        if item == b' ' {
            return &s[..i];
        }
    }

    // if there's no spaces, return entire string
    &s
}

// can pass a `String`, a reference to a `String`, slices of string literals,
// or string literals. much more flexibility
fn first_word_proper_signature(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
