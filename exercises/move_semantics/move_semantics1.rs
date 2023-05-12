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
}

// I AM NOT DONE
