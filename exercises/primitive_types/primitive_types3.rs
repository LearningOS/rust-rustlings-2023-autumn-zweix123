// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = [42; 100];
    // 关于数组的创建
    // 1. 直接使用[a, b, c]
    // 2. 使用语法[num; len], 中间是分号, 语义还是有点怪的, 总要反应下哪个是长度

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
