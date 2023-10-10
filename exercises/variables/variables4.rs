// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line

    // Rust默认常量, 通过关键字mut使之可修改
    println!("Number {}", x);
}
