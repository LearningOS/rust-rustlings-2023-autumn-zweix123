// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

use std::cmp;
// 学到了Rust的一个库
// 而且Rust应该是没有三目运算符

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    cmp::max(a, b)
}

// Don't mind this for now :)
// 我们看看下面这些东西, 虽然上面说不用看
// 是Rust的attribute属性，我理解对应其用法即可
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
