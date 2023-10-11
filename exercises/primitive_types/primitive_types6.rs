// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;
    // 这里展示了tuple的访问方式, 至少比C++阳间写

    assert_eq!(2, second, "This is not the 2nd number in the tuple!");

    // let mut a = (1, 2, 3);
    // a.1 = 4;
    // print!("{} {} {}", a.0, a.1, a.2);
    // 哦!, 也是能修改的
}
