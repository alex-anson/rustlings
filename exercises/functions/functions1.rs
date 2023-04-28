// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me();

    // so Rust has some sort of hoisting mechanism
    fn call_me() {
        println!("ayo")
    }
}
