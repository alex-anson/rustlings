// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
    println!("");
    println!("");

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("");

    let a: Point<f32> = Point { x: 1.2, y: 3.4 };
    println!("{}", a.distance_from_origin());
    // .distance_from_origin() is not available on variable p.
    // this method will only be available on instances of type Point<f32>.
}

/*
performance
monomorphization

Monomorphization is the process of turning generic code into specific code by
filling in the concrete types that are used when compiled.
*/
