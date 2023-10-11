// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// 原来Rust的宏需要前后的声明顺序？为什么普通的函数或者结构体不需要, 为什么这么设计?

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
