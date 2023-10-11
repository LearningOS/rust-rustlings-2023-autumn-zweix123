// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// 下面很能体现Rust的风格
// 如果从C++的视角, file_me的第一句代码是一个拷贝函数(深拷贝)
// 但是在Rust是将vec0的所有权转移到fill_vec参数中, 在转移到函数中的vec, 在返回到主函数的vec1, 从始至终只有一块堆内存
// 紧紧把握这点即可知mut何时添加

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
