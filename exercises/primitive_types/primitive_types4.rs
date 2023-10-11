// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    // 使用超尾
    // 切片, 切片肯定是一种引用, 所以需要取地址符号
    // 还记得字符串切片么?

    assert_eq!([2, 3, 4], nice_slice)
}
