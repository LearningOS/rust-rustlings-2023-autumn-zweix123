// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// 书接上回, 我们发现只有一块内存转移来转移去, 所以正常调用后vec0就“空了”, 但是下面的代码在调用函数后依然使用vec0这个已经被转移的变量, 就出现错误了
// 这里有两种改法, 一个是函数参数声明为引用, 在函数中进行clone,
// 或者直接在调用是clone
// 指的注意的是, 如果采用第一种方法, 由于vec0没有调用push, 所以不能推测其成员类型
// 关于第二种做法, 还能再调整, 比如直接参数就是可变的, 内部就不用重新赋值(本质是改变变量可修改性)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    println!(
        "{} has length {}, with contents: `{:?}`",
        "vec0",
        vec0.len(),
        vec0
    );

    vec1.push(88);

    println!(
        "{} has length {}, with contents `{:?}`",
        "vec1",
        vec1.len(),
        vec1
    );
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
