// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

fn main() {
    let a: [&str; 112] = [""; 112];
    println!("{}", a.len());

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }

    diff_nums();
}

// working this out was surprisingly difficult. i'm guessing there's an easier way.
fn diff_nums() {
    let mut a: [i32; 6] = [1; 6];

    let mut u: usize = 0;
    let mut i: i32 = 0;
    for mut el in a {
        a[u] = i;
        u += 1;
        i += 1;
    }
    println!("{:?}", a) // [0, 1, 2, 3, 4, 5]
}
