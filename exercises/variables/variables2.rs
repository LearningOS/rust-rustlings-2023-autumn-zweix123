// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x = 10;
    // 同样的, 既然是常量, 就必须要初始化
    // let mut x;
    // 哪怕是变量, 也要初始化, 因为如果没有初始化就有歧义
    // 避免歧义也是Rust安全的一个方法
    // let x: i32;
    // 但是这样是可以编译的, 估计是有默认初始化
    // let x: i32;
    // println!("{}", x);
    // 但是这样反而不行, 即不使用默认初始化?
    // 或者我这么理解, 关键在于上下文, 一旦变量被使用, 它就应该是有值的
    // 但是如果不使用类型提示, 且不初始化, 那么这个变量有歧义, 就是究竟是什么类型的? 即使它没有被使用
    // 反过来没有提供类型的真的是没有类型么? 不是,  而是类型足够推导
    // 核心还是歧义吧
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
