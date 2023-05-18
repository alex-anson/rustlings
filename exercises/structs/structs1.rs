// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

/* SUMMARY (The Book)
Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In impl blocks, you can define
functions that are associated with your type, and methods are a kind of associated
function that let you specify the behavior that instances of your structs have.
*/

struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitLikeStruct;

/* THE BOOK */
struct Rectangle {
    width: u32,
    height: u32,
}

// you're allowed to have multiple `impl` blocks for one struct. useful for
// generic types and traits.
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // pass a reference - we don't want ownership of `other`. we aren't mutating
    // it; we don't need a mutable borrow.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // ASSOCIATED FUNCTION
    // doesn't need an instance of the Rectangle type to work with.
    // "Associated functions that aren’t methods are often used for constructors
    // that will return a new instance of the struct."
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // associated functions that return a new instance of the struct are often
    // called "new" - but "new" isn't a special name / built into the language
    // ^ i'm guessing this is convention
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn can_i_do_this(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!();
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!();
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // &rect2 (and &rect3) => passing an immutable borrow. can use rect2 (and 3) again
    // after invoking the can_hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!();

    let mut square1 = Rectangle::square(3);
    square1.width = 12;
    println!("square1's new width is {}", square1.width);

    let new_rect = Rectangle::new(40, 30);
    println!("the area of new_rect is {}", new_rect.area());

    let yeah = Rectangle::can_i_do_this(32);
    println!("yeah's height {}", yeah.height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }

    #[test]
    fn easy_fail() {
        main();
        assert_eq!(12, 12)
    }

    // #[test]
    // fn run_it() {
    //     let func = run_it();

    //     assert_eq!(func, 12)
    // }
}

// fn run_it() -> u8 {
//     let story = "Once upon a time...";

//     let ptr = story.as_ptr();
//     let len = story.len();

//     // story has nineteen bytes
//     println!("{}", ptr);

//     11
// }
