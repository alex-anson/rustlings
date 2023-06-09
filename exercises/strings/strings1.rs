// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    whhht();
    println!();
    example_two();
}

fn current_favorite_color() -> String {
    String::from("teal")
}

fn whhht() {
    let hello = "Здравствуйте";
    // let one = &hello[0];
    let one = &hello.bytes().nth(0);

    println!("aayyooo {}", hello.len()); // prints 24 --
                                         // even though it's 12 characters long, it
                                         // takes 24 bytes to encode it in UTF-8.

    println!("result... {:?}", one);
    // ^ prints result... Some(208)

    // (when encoded in UTF-8, the first byte is 208.)
}

/*
- strings are implemented as a collection of bytes, plus some methods to provide
useful functionality when those bytes are interpreted as text.

- since the String type is implemented with vectors under the hood, can grow/shrink
/manipulate the contents
*/

fn example_two() {
    let hello = "hello";
    // let one = &hello[0];
    // ^ if this didn't make the compiler throw an error, "one" would contain the
    // value 104. since that's probably not what humans would want when they're
    // asking for the first letter of a string, Rust prevents the code from compiling
    let one = &hello.bytes().nth(0);

    println!("result... {:?}", one);
    // ^ prints result... Some(104)

    match one {
        Some(x) => println!("result... {:?}", x),
        // ^ prints result... 104
        None => println!("nope"),
    }

    // (when encoded in UTF-8, the first byte is 104. hence the result)
}
