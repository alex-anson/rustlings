// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x: i32 = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);

    let spaces: &str = "   ";
    println!("{}", spaces);
    // hmm what IS `usize`?
    let spaces: usize = spaces.len();
    println!("{}", spaces);
    println!();

    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("{number:0>width$}", number = 7, width = 3);
}
