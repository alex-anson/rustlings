// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

struct CoolStruct {
    num: i32,
    ans: bool,
}

fn main() {
    call_me(3, true);

    ayo(CoolStruct { num: 12, ans: true })
}

fn call_me(num: i32, ans: bool) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn ayo(CoolStruct { num, ans }: CoolStruct) {
    println!("rust is interesting.")
}
