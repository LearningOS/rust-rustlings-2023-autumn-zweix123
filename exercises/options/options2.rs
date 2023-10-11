// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// 这里展示Rust中if语句和while语句的自由度
// 还要结合Option的None属性
// 还有就是Option的“拆包”

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // 我们希望将一个Option的值取出, 可以使用下面的语法
        // let Sone(l) = r;  // r is a option
        // 我们发现r可能没有值, 怎么检测呢?
        // 可以直接将其放在if语句中
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 为什么这里需要呢? 因为一个线性表可能是空的, 所以标准库中的pop返回的是option
        // 所以下面的拆包使用的两层Some,
        // 同时也能套进循环中
        // 这太甜了
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
