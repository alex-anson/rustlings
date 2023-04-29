// primitive_types1.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening: bool = false; // Finish the rest of this line like the example! Or make it be false!
    if is_evening {
        println!("Good evening!");
    }

    if let is_afternoon = true {
        println!("afternoon")
    }

    println!();
    // EXPONENTS
    println!("{}", 2_u32.pow(7)); // is equivalent to...
    let x: u32 = 2;
    println!("{}", x.pow(7));
    // the type MUST be explicit
    let x: u8 = 255; // highest an unsigned 8bit can go -- 2.pow(8) - 1

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 ONLY
    let floating = 123_456.78;

    let z = 'é';
}

// a scalar type represents a single value.
// rust has 4 primary scalar types: integers, floats, booleans, characters.
//
