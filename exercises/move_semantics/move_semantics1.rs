// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

/*
All data stored on the stack must have a known, fixed size. Data with an unknown
size at compile time or a size that might change must be stored on the heap instead.
*/

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    println!();
    ownership();
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn ownership() {
    // string literal -> let s = "hello";
    /*
    looking at how rust knows when to clean up data stored on the heap.

    the String type. mutable, not (necessarily) known at compile time (i.e.,
    accept/store user input).
    */
    let mut s = String::from("hello");
    /*
    ^ ^ ^ "The double colon `::` operator allows us to namespace this particular
    `from` function under the `String` type rather than using some sort of name
    like `string_from`."
    */
    s.push_str(", world <3");
    println!("{}", s);
    // string literals can't be mutated like this ^

    /*
    the difference is how these two types deal with memory --

    since the contents of a string literal are known at compile time, the text is
    hardcoded directly into the final executable.

    String type - to be able to support a mutable, growable piece of text, we need
    to allocate an amount of memory on the heap, unknown at compile time, to hold
    the contents. this means --
    - the memory must be requested from the memory allocator at runtime.
    - we need a way of returning this memory to the allocator when we're done with
    our `String`.
    part one - we call `String::from`, and its implementation requests the needed memory.
    part two - rust's approach is different than other languages that don't have
    garbage collectors - the memory is automatically returned once the variable
    that owns it goes out of scope. rust calls a special ƒn for us, `drop`. it's
    called at the closing curly bracket. (seems simple, but can cause unexpected
    behavior in more complicated situations.)
    */

    let s1 = String::from("hello");
    let s2 = s1;
    /*
    the second line doesn't exactly make a copy of the value in s1 and bind it
    to s2...
    - a String is made up of three parts: a pointer to the memory that holds the
    contents of the string, a length, and a capacity. this group of data is stored
    on the stack. the heap holds the contents of s1.
    length = how much memory, in bytes, the contents of `String` are currently using.
    capacity = total amount of memory, in bytes, that the string has received from
    the allocator.
    - s1 and s2 share the data on the heap, that data is not copied. s2 has its own
    pointer (which points to the same location in memory as does s1), length, and
    capacity.
    ... sounds similar to how objects in javascript work. (except not now that i've
    continued reading!)
    - to avoid causing a "double free" error that would occur if rust called the
    `drop` function twice on the same memory (the same data on the heap), to ensure
    memory safety, rust considers s1 no longer valid right after the `let s2 = s1;`
    line.
    - the "double free" error is a memory safety bug. freeing memory twice can lead
    to memory corruption, which can potentially lead to security vulnerabilities.
    */
    // println!("this will trigger an error {}", s1);
    // INSTEAD OF IT BEING CALLED A SHALLOW COPY, IT'S CALLED A MOVE. because of
    // the invalidation of the first variable.

    // if you DO want to deeply copy heap data (which is expensive in terms of memory
    // and performance), use the `clone` method --
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    ownership_and_functions();
    return_values_and_scope();
    return_multiple_values_with_tuple();
}

// OWNERSHIP AND FUNCTIONS
// passing a variable to a function will move or copy, just as an assignment does.
fn ownership_and_functions() {
    let s = String::from("🔥🔥🔥"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    /*
        ^ ^ ^ ⭐️ this definitely feels weird to me! (just new, really) ... it makes
        complete sense when you understand how the stack and heap operate and how
        memory is freed.
    */

    let x = 5_000; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// RETURN VALUES AND SCOPE
// returning values can also transfer ownership
fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    println!("{} - getting ownership 💋", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("rust is so cool... {}", s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(mut a_string: String) -> String {
    // a_string comes into scope
    a_string.push_str(", worlddd");

    a_string // a_string is returned and moves out to the calling function
}
/*
When a variable that includes data on the heap goes out of scope, the value will be
cleaned up by drop unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function
is a bit tedious. What if we want to let a function use a value but not take
ownership?*️⃣ It’s quite annoying that anything we pass in also needs to be passed
back if we want to use it again, in addition to any data resulting from the body
of the function that we might want to return as well.

*️⃣ achieve this with REFERENCES.

- can return multiple values using a tuple:
*/
fn return_multiple_values_with_tuple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
