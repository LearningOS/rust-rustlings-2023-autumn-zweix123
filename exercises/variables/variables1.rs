// variables1.rs
//
// Make me compile!
//
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // let x = 5;
    // rust定义“变量”使用关键字let
    // 但是默认是不可变的, 这种不可变性是Rust较于其他语言较大的不同
    // x = 1;  // 这就是不行
    // let mut x = 42;
    // 这个样子才可以修改
    let mut x = 1;
    x = 2;
    println!("x has the value {}", x);
}
