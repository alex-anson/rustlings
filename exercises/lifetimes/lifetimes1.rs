// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk
// of going out of scope before it is used. Remember, references are borrows
// and do not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a hint.

// this example was taken directly from The Book

/*
fuuuck this is confusing stuff.

The help text reveals that the return type needs a generic lifetime parameter on
it because Rust can’t tell whether the reference being returned refers to x or y.
Actually, we don’t know either, because the if block in the body of this function
returns a reference to x and the else block returns a reference to y!

When we’re defining this function, we don’t know the concrete values that will be
passed into this function, so we don’t know whether the if case or the else case
will execute. We also don’t know the concrete lifetimes of the references that will
be passed in, so we can’t look at the scopes as we did in Listings 10-17 and 10-18
to determine whether the reference we return will always be valid. The borrow
checker can’t determine this either, because it doesn’t know how the lifetimes of
x and y relate to the lifetime of the return value. To fix this error, we’ll add
generic lifetime parameters that define the relationship between the references
so the borrow checker can perform its analysis.

Lifetime annotations don’t change how long any of the references live. Rather, they
describe the relationships of the lifetimes of multiple references to each other
without affecting the lifetimes. Just as functions can accept any type when the
signature specifies a generic type parameter, functions can accept references with
any lifetime by specifying a generic lifetime parameter.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    /*
    ^ ^ ^ the signature expresses the following constraint: the returned
    reference will be valid as long as both the parameters are valid
    */
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
The function signature now tells Rust that for some lifetime 'a, the function
takes two parameters, both of which are string slices that live at least as long
as lifetime 'a. The function signature also tells Rust that the string slice
returned from the function will live at least as long as lifetime 'a.

REMEMBER, WHEN WE SPECIFY THE LIFETIME PARAMETERS IN THIS FUNCTION SIGNATURE,
WE’RE NOT CHANGING THE LIFETIMES OF ANY VALUES PASSED IN OR RETURNED. RATHER,
WE’RE SPECIFYING THAT THE BORROW CHECKER SHOULD REJECT ANY VALUES THAT DON’T
ADHERE TO THESE CONSTRAINTS.

Note that the longest function doesn’t need to know exactly how long x and y will
live, only that some scope can be substituted for 'a that will satisfy this signature.
*/

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

/*
https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
One detail we didn’t discuss in the “References and Borrowing” section in Chapter
4 is that every reference in Rust has a lifetime, which is the scope for which
that reference is valid. Most of the time, lifetimes are implicit and inferred,
just like most of the time, types are inferred. We only must annotate types when
multiple types are possible. In a similar way, we must annotate lifetimes when the
lifetimes of references could be related in a few different ways. Rust requires us
to annotate the relationships using generic lifetime parameters to ensure the actual
references used at runtime will definitely be valid.
*/

/*
a reference to an i32 without a lifetime parameter, a reference to an
i32 that has a lifetime parameter named 'a, and a mutable reference to
an i32 that also has the lifetime 'a.

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/
