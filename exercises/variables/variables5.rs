// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: i32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);

    // don't *have to* type variables declared with `let` (do have to with `const`)
    let number = 9;
    {
        // INNER SCOPE
        let number = number + 3;
        println!("inner scope value of number {}", number)
    }

    println!("scoped ^");
    println!("value in outer scope {}", number);
}
