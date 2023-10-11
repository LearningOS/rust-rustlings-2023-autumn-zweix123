// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// 书接上回，继续讨论option相关问题
// 下面为什么需要ref，是因为match语句后将y作为返回值了，即这个值后面还要用，
// 如果前面没有ref，则其值就被移动p中了，不对。

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.

    // match y {
    //     Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //     _ => panic!("no match!"),
    // }
    // y; // Fix without deleting this line.
}
