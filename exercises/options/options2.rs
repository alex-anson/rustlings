// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word, Some(target));
            // definitely feels like i've done this the wrong way. it's hard to
            // discern what exactly they're looking for in this exercise...
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(range));
            range -= 1;
        }
        // yeah i'm confused. gonna need to spend some time on this when i start
        // actually going through The Book
    }
}
