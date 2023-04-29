// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)
// I AM NOT DONE
fn main() {
    let original_price: i32 = 112;
    println!("Your sale price is {}", sale_price(original_price));

    println!();
    another_fn();
    println!();
    another_another();
    println!();
    while_loop();
    println!();
    equivalent_while();
}

fn another_fn() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // this value will be stored in `result`
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// multiple loops? (optionally) label them.
fn another_another() {
    let mut count = 0;
    // loop labels must begin with a single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // this break statement applies to the unnamed loop, because it's
                // the innermost loop
                break;
            }
            if count == 2 {
                // this break statement applies to the named loop, because we said so
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number > 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("🚀")
}
// equivalent without using while
fn equivalent_while() {
    println!("begin equiv ƒn");
    let mut number = 3;

    loop {
        if number > 0 {
            println!("{number}!");
            number -= 1;
        } else {
            break println!("🚀");
        }
    }
}

// so this is how you type a return value.
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
