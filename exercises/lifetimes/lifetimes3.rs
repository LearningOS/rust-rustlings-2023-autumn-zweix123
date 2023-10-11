// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// 一个新的感悟是这个生命周期只和引用有关系
// 如果是一个变量由于Rust的默认移动所有拥有唯一所有权
// 只有引用可能会导致引用悬空

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
