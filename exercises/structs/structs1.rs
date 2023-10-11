// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// 这个练习展示了结构体相关的一些东西
// 比如定义方式
// 比如初始化方式
// 和tuple的对比
// 默认打印: 给类提供属性#[derive(Debug)], 然后使用format宏的"{:?}"

#[derive(Debug)]
struct ColorClassicStruct {
    // TODO: Something goes here
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = (0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        let dummy = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(message, "UnitLikeStructs are fun!");

        let message = format!("{:?}s are fun!", dummy);
        println!("{}", message);
    }
}
