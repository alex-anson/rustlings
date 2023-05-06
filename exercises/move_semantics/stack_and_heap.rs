/*
https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
^ 2nd article of today. these notes are from the section where they give a brief
overview of the stack and heap.


STACK => LIFO. stack of plates.
- all data stored on the stack *must* have a known, fixed size.
- terms: "pushing onto the stack" (adding data), "popping off the stack" (removing data)

HEAP => not as organized. you're requesting a certain amount of space.
- memory allocator finds an empty spot that's big enough, marks it as being in
use, and returns a pointer (which is the address of that location).
- term for this ^: "allocating on the heap" or just "allocating"
- the pointer IS a known, fixed size, so it can be stored on the stack. but when
you want the actual data, you need to follow the pointer.


the main purpose of ownership is to manage heap data.

*/

// https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html

fn foo(x: &i32) {
    // x (the arg ^), y & z ... pushed onto the stack.
    // reading the second article now ... i think that even though x and y are
    // pointers, they're still considered to be on the stack.

    // x has same value as j (since that's what was passed) - a pointer
    let y = 10;
    let z = &y;

    /*
    before calling baz, the stack/heap looks something like this =>
        Address 	Name	Value
        (2^30) - 1		    20
        ...	        ...	    ...
        5	        z	    → 4
        4	        y	    10
        3	        x	    → 0
        2	        j	    → 0
        1	        i	    → (2^30) - 1
        0	        h	    3
    */

    baz(z);
    bar(x, z);
}

/*
when foo calls bar =>
Address 	Name	Value
(2^30) - 1		    20
(2^30) - 2		    5
...	        ...	    ...
10	        e	    → 9
9	        d	    → (2^30) - 2
8	        c	    5
7	        b	    → 4
6	        a	    → 0
5	        z	    → 4
4	        y	    10
3	        x	    → 0
2	        j	    → 0
1	        i	    → (2^30) - 1
0	        h	    3
*/

/*
upon `return`ing from bar =>
Address 	Name	Value
(2^30) - 1		    20
...	        ...	    ...
5	        z	    → 4
4	        y	    10
3	        x	    → 0
2	        j	    → 0
1	        i	    → (2^30) - 1
0	        h	    3
*/
fn bar(a: &i32, b: &i32) {
    let c = 5;
    let d = Box::new(5);
    let e = &d;

    baz(e);
}

fn baz(f: &i32) {
    // f & g pushed onto the stack.
    let g = 100;
} // <- baz's stack frame is gone now (f & g popped off of the stack). stack/heap
  // looks like it did before we called baz

// main goes first...
fn main() {
    // j & h are pushed onto the stack. i is allocated on the heap.
    // (i's pointer is stored on the stack. i's value is on the heap.)
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo(j);
}
