// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // let {name, age} = cat; ...nope
    // let [name, age] = cat; ...nope
    // they call this destructuring 🍫   yay for js similarities
    let (name, age) = cat; // ...bingo

    println!("{} is {} years old.", name, age);
}
