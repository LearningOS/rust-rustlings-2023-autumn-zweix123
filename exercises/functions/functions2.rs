// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

// 这里涉及两个语法
// 一个是关于函数参数, yeah, 比较直觉
// 另一个是循环, 这个确实比较独特, whatever, remember it.
// 应该是对这些整数类型编译器有一个判断, start..end只支持整数

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
