// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

/*
"The tests module is a regular module that follows the usual visibility rules
we covered in Chapter 7 in the “Paths for Referring to an Item in the Module
Tree” section. => https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
*/
#[cfg(test)]
mod tests {
    use super::*;
    // ^ brings everything from the outer scope into the scope of this
    // inner module.

    #[test]
    fn is_true_when_even() {
        let result = is_even(12);
        assert!(result);
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
