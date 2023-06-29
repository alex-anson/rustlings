/* https://doc.rust-lang.org/rust-by-example/std/option.html

The Option<T> enum has two variants:
    None, to indicate failure or lack of value, and
    Some(value), a tuple struct that wraps a value with type T.
*/

// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    let invalid_time: Option<u16> = Some(23);

    while let Some(x) = Some(time_of_day) {
        if Some(x) > invalid_time {
            return None;
        } else if Some(x) > Some(21) {
            return Some(0);
        } else {
            return Some(5);
        }
    }
    Some(12) // eeep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
