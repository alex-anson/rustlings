// use console::{style, Emoji};
// use crate::console;
// use crate::style;

// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *i *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|num| {
            // TODO: Do the same thing as above - but instead of mutating the
            // Vec, you can just return the new number!
            2 * num
        })
        .collect()
}

fn vec_map_again(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|num| 2 * num).collect()
}

// it's gonna be really cool to come back to this code when i know what i'm doing 😆
// macro_rules! sweet {
//     ($fmt:literal, $ex:expr) => {{
//         let formatstr = format!($fmt, $ex);
//         println!(
//             "{} {}",
//             style(Emoji("✅", "✓")).green(),
//             style(formatstr).green()
//         );
//     }};
// }

// More iterating, following The Book
fn it() -> i8 {
    // let formatstr = format!(fmt: literal, ex: expr);
    let (x, y) = (1, 2);

    // for loop to get immutable references to each element
    let b = vec![100, 32, 57];
    for i in &b {
        println!("{i}");
        // sweet!("legggooo {}", x)
    }

    // easy to make test pass or fail. (failing test = will print my `println`s)
    12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map_again() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map_again(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn leeggo() {
        let b = it();
        assert_eq!(12, b);
    }
}
