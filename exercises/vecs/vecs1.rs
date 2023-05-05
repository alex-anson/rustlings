// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

use std::vec;

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array

    // WAY ONE
    let mut v: Vec<i32> = Vec::new();
    for el in a {
        v.push(el);
    }

    // WAY TWO
    let mut w = vec![10, 20, 30];
    w.push(40); // can still use the push method

    (a, w)
}

fn other_fn() {
    // two ways to reference/access a value stored in a vector
    let a = [1, 2, 3, 4, 5];

    // WAY ONE - indexing syntax
    let third = &a[2]; // & + [] gives us a reference to the element at the provided index
    println!("third element is {third}");

    // WAY TWO - `get` method
    let fourth = a.get(3);
    match fourth {
        Some(fourth) => println!("fourth element is {fourth}"),
        None => println!("there is no fourth element"),
    }
    // ^ handling the panic scenario (attempting to access an element at a position
    // that doesn't exist) instead of crashing the app
}

fn cant_borrow_it() -> i32 {
    // can't have mutable and immutable references in the same scope.
    let mut a = vec![1, 2, 3, 4, 5];

    // immutable reference to the first element
    let first = &a[0]; // "immutable borrow occurs here"
    println!("first element: {first}"); // this is fine

    a.push(6); // "mutable borrow occurs here"
               // println!("first element: {first}"); // <- this triggers the panic
               //                      " ----- immutable borrow later used here "

    /*
    This error is due to the way vectors work: because vectors put the values next
    to each other in memory, adding a new element onto the end of the vector might
    require allocating new memory and copying the old elements to the new space,
    if there isn’t enough room to put all the elements next to each other where
    the vector is currently stored. In that case, the reference to the first element
    would be pointing to deallocated memory. The borrowing rules prevent programs
    from ending up in that situation.
    */

    println!("whole thing {a:?}");
    // just wanted to make the tests different
    12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }

    #[test]
    fn test_fir_fun() {
        let () = other_fn();
        assert_eq!(12, 12);
        let a = cant_borrow_it();
        assert_eq!(12, a);
    }
}
