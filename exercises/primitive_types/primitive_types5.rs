// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // 这玩意在不同语言中的名字是一样的，tuple元组
    let (name, age)/* your pattern here */ = cat;
    // 哇偶, 结构化绑定

    println!("{} is {} years old.", name, age);
}
