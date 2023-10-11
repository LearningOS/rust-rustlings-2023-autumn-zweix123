// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    println!("{}", foo());
}

fn current_favorite_color() -> String {
    // 两种用法
    // String::from("Blue")
    "Blue".to_string()
    // 真妙呀, 我可以自信的知道东西是移动出去了, 这性能必须高呀
}

// 其实还有做法
fn foo() -> &'static str {
    "Blue"
}

// 那么这里的话, 我个人觉得是区分这样
// 比如这里的"Blue"是在编译期知道的, 没必要放在堆里面, 这里就表明是静态的变量
// 其实这个编译器知道, 但是就是强制程序员必须用这个关键字, 让程序员知道这个玩意在哪里
