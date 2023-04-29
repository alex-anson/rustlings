// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me();

    // so Rust has some sort of hoisting mechanism
    fn call_me() {
        println!("ayo")
    }

    let t = inner_scope_is_an_expression();
    // t
}

fn inner_scope_is_an_expression() -> usize {
    let y = {
        let x = 3;
        x * 4
    };

    /**
     ⌄ this part is an expression. it evaluates to a value.

    {
        let x = 3;
        x * 4
    };

    evaluates to 12, which then gets bound to `y` as part of the `let` statement.
    */

    println!("The value of y is: {y}");
    12
}
