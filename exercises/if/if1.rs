// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

pub fn bigger(a: i32, b: i32) -> i32 {
    // notice that nothing gets printed. calling `something` here and in one of
    // the test cases. will only print if one of the tests fail, or if you pass
    // --nocapture on the command line when running the tests.
    something(12, '😜');
    if a > b {
        a
    } else {
        b
    }
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
}

fn something(val: i32, unit: char) {
    println!("{}{}", val, unit)
}

// YAY TESTS!!!!!!!!!!!!!!!!!!!!!!! 🔥🔥🔥
// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
        something(12, '😜')
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
