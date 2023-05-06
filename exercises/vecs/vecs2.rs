// use console::{style, Emoji};
// use crate::console;
// use crate::style;

// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|num| {
            // TODO: Do the same thing as above - but instead of mutating the
            // Vec, you can just return the new number!
            2 * num
        })
        .collect()
}

fn vec_map_again(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| 2 * num).collect()
}

// it's gonna be really cool to come back to this code when i know what i'm doing 😆
// macro_rules! sweet {
//     ($fmt:literal, $ex:expr) => {{
//         let formatstr = format!($fmt, $ex);
//         println!(
//             "{} {}",
//             style(Emoji("✅", "✓")).green(),
//             style(formatstr).green()
//         );
//     }};
// }

// More iterating, following The Book
fn it() -> i8 {
    let (x, y) = (1, 2);

    // for loop to get immutable references to each element
    let b = vec![100, 32, 57];
    for i in &b {
        println!("{i}");
        // sweet!("legggooo {}", x)
    }

    // unrelated to the actual book exercise
    let mut a = vec![1, 2, 3];

    // iterate over mutable references to each element in a mutable vector
    let mut c = vec![100, 32, 57];
    for i in &mut c {
        *i += 50;
        // to change the value that the mutable reference refers to, we have to use
        // the `*` dereference operator to get to the value in `i` before we can use
        // the `+=` operator.

        a.push(87)
    }
    println!("a is... {a:?}");

    // like any other `struct`, a vector is freed when it goes out of scope. the
    // borrow checker ensures that any references to contents of a vector are only
    // used while the vector itself is valid
    {
        let e = vec![3, 6, 9];

        // do stuff with e
    } // <- e goes out of scope here.

    // easy to make test pass or fail. (failing test = will print my `println`s)
    12
}

/*
Vectors can only store values that are the same type. This can be inconvenient;
there are definitely use cases for needing to store a list of items of different
types. __Fortunately, the variants of an enum are defined under the same enum
type,__ so when we need one type to represent elements of different types, we can
define and use an enum!
https://doc.rust-lang.org/stable/book/ch08-01-vectors.html#using-an-enum-to-store-multiple-types
*/

// use std::fmt::Debug;
// use std::fmt::Formatter;
fn book_enum_vec_intro() -> i8 {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Text(String::from("hello world")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(9.12),
    ];

    // impl Debug for SpreadsheetCell {
    //     fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    //         // allows the code to compile
    //         todo!()

    //         // some of my attempts
    //         // (Ok(println!("{row:?}")), Err())
    //         // Ok((|| row)())
    //     }
    // }
    // ended up implementing this with a single line: #[derive(Debug)]

    println!("{row:?}");

    for i in row {
        println!("{:?}", i);
    }

    12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map_again() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map_again(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn leeggo() {
        let b = it();
        assert_eq!(12, b);
    }

    #[test]
    fn a() {
        let b = book_enum_vec_intro();
        assert_eq!(12, b);
    }
}
