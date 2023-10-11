// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// 这里又遇到个知识点, 就是unwrap()
// 因为之前从这里拿值的方式太Modern, 传统上还是习惯用函数调用的方式
// 就是这个玩意, 针对Option可能是Node，针对Result可能是第二个参数
// 递归向上继续触发

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // 我们看这里的实现
    // 对帅string的迭代单位自然就是char
    // 这里的迭代器, 至少是我的实现更像是分支循环,
    // 如果进来空的, 则返回空的
    // 如果不是, 则地铁一个特殊处理
    // 后面的手动extend, 这个方法自动的一个一个放进去
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut str = String::with_capacity(input.len());
            str.push(first.to_uppercase().next().unwrap());
            str.extend(c);
            str
        }
    }
    // 这理解这里应该有一个虚拟头, 只有调用next才到一个真的有意义值
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
    // 来点函数式震撼
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let capitalized_words: Vec<String> = words.iter().map(|&word| capitalize_first(word)).collect();
    capitalized_words.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
