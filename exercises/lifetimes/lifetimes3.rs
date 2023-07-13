// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
// ^ NEEDED. can't define a struct that holds a reference without using explicit lifetimes
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);

    println!();
    println!();
    again();
    println!();
    println!();
    let k = "lifetimes definitely feel weird..";
    let n = first_word(k);
    println!("{}", n)
}

struct ImportantExcerpt<'a> {
    // ^ ^ ^ this means that an instance of ImportantExcerpt can’t outlive the
    // reference it holds in its `part` field.
    part: &'a str,
}

fn again() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part)
}

// why does this compile without lifetime annotations?
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
/* answer:
The reason this function compiles without lifetime annotations is historical: in
early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every
reference needed an explicit lifetime.

After writing a lot of Rust code, the Rust team found that Rust programmers were
entering the same lifetime annotations over and over in particular situations. These
situations were predictable and followed a few deterministic patterns. The developers
programmed these patterns into the compiler’s code so the borrow checker could infer
the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.
*/

/* The Static Lifetime

One special lifetime we need to discuss is 'static, which denotes that the
affected reference can live for the entire duration of the program. All string
literals have the 'static lifetime, which we can annotate as follows:

let s: &'static str = "I have a static lifetime.";

The text of this string is stored directly in the program’s binary, which is
always available. Therefore, the lifetime of all string literals is 'static.

*/
