// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

/*
NOTE: FROM THE BOOK
https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
generally want to return the error to the caller, so it can decide the right way to respond
*/

// As with `Option`, we can use combinators such as `map()`.
// This function reads:
// Multiply if both values can be parsed from str, otherwise pass on the error.
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("Number is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("");
    println!("");
    println!("");

    let twenty = multiply("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply("t", "2");
    print(tt);

    println!("");
    println!("");
    println!("");
}

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }

    #[test]
    fn negative_batman() {
        main();
        assert_eq!(12, 12)
    }
}
