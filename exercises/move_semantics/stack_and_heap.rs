// https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html

fn foo(x: &i32) {
    // allocate memory for x (the arg ^), y & z.
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
    // memory allocated for f & g
    let g = 100;
} // baz's stack frame is gone now. stack/heap looks like it did before we called baz

// main goes first...
fn main() {
    // allocate memory for j, i, & h.
    // i is on the heap, and so has a value pointing there.
    let h = 3;
    let i = Box::new(20);
    let j = &h;

    foo(j);
}
