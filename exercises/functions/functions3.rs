// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    // this one was *too* easy after what i did for the 2nd one
    call_me(12);
    println!();

    let s = five();
    println!("The value of s is: {s}");
    println!();
    fn five() -> i32 {
        5
    }

    println!("{}", cool_fn())
}

// "expected named lifetime parameter"
// help: this function's return type contains a borrowed value, but there is no
// value for it to be borrowed from
// help: consider using the `'static` lifetime
// fn cool_fn() -> &str {
fn cool_fn() -> &'static str {
    "rust is really cool 🦀"
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
