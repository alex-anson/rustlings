// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.
use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ⌄ ⌄ ⌄ shows that the lifetime of the reference in result must be the smaller
// lifetime of the two arguments.           ... doesn't quite make sense to me
// ORIGINAL FUNCTION =>
// fn main() {
//     let string1 = String::from("long string is long");           // ----------------+- 'b
//     let result;                                                  // ---------+- 'c  |
//     {                                                                        |      |
//         let string2 = String::from("xyz");                       // -+- 'd   |      |
//         result = longest(string1.as_str(), string2.as_str());        |       |      |
//     }                                                               -+       |      |
//     println!("The longest string is '{}'", result);                          |      |
// }                                                                // ---------+------+

//                   illustrates the lifetimes of each of the variables  ^ ^ ^

fn main() {
    let string1 = String::from("long string is long");
    let result;

    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());

    println!("The longest string is '{}'", result);

    println!();
    println!();
    again();
}

fn again() {
    let d = "hello";
    {
        let e = "world";
        let f = longest(d, e);
        println!("it's a cold {}", f)
        // e is valid until the end of the inner scope (here, basically)
        // f references something (e) that is valid until the end of the inner scope
    }
    // d is valid until the end of the outer scope (here, basically)
}

/*
(the following compiles)

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

We’ve specified a lifetime parameter 'a for the parameter x and the return
type, but not for the parameter y, because the lifetime of y does not have
any relationship with the lifetime of x or the return value.
*/

// IMPORTANT:
// generics, trait bounds, and explicit lifetimes... TOGETHERRR
//                               ⌄---------⌄-----------⌄-- input lifetimes  ⌄---- output lifetime
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    // where clause                 ^ generic type parameter
    T: Display, // whatever type T is, it must implement the Display trait.
{
    // ^ trait bound
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
<'a, T>

Because lifetimes are a type of generic, the declarations of the lifetime
parameter 'a and the generic type parameter T go in the same list inside
the angle brackets after the function name.
*/
