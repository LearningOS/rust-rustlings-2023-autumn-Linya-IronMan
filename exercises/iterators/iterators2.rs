// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

use std::ascii::AsciiExt;

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
// NOTE: string 类型字符串拼接 +
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_ascii_uppercase().to_string() + &input[1..],
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
// NOTE: 直接修改传入的参数是不好的
// map 根据一个 iter 生成一个新的 iter（迭代器）
// 将一个可迭代对象通过一个函数生成另外一个可迭代对象
// NOTE: Collect 将一个迭代器搜集起来，生成vector
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|x| capitalize_first(x)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
// NOTE: collect 会返回一个不确定的类型 B，需要手动明确类型 可以指定最后搜集的 Vector 类型
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|x| capitalize_first(x))
        .collect::<Vec<String>>()
        .join("")
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
