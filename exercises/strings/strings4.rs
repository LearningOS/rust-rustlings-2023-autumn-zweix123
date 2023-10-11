// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    let s = string_slice;
    let S = string;

    s("blue");
    S("red".to_string());
    S(String::from("hi"));

    S("rust is fun!".to_owned());
    // 相当于String::from(它转移所有权哦)

    S("nice weather".into());
    // 相当于to_owned, 要求实现某个trait

    S(format!("Interpolation {}", "Station"));

    s(&String::from("abc")[0..1]);
    s("  hello there ".trim());
    S("Happy Monday!".to_string().replace("Mon", "Tues"));
    S("Happy Monday!".replace("Mon", "Tues")); // 这里也被强转了

    S("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 这里调用to_...被强转了么?
}
