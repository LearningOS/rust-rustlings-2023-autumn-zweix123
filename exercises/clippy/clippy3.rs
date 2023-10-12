// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// 你学废了嘛？

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 可能下面才是推荐的写法吧
    if let Some(temp) = my_option {
        println!("{:?}", temp);
    } else {
        assert!(my_option.is_none());
        println!("my option is None");
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(1, 5);
    // .resize(0, 5);  这个接口的第一个参数是新长度, 第二个参数是默认值大概
    println!("This Vec is empty, see? {:?}", my_empty_vec.is_empty());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    // 参加使用&mut, 而不仅仅是&
    // 好好好, 和传引用也默认的const T&是吧
    println!("value a: {}; value b: {}", value_a, value_b);
}
