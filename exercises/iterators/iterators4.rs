// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// pub fn factorial(num: u64) -> u64 {
//     // Complete this function to return the factorial(阶乘) of num
//     // Do not use:
//     // - return
//     // Try not to use:
//     // - imperative style loops (for, while)
//     // - additional variables
//     // For an extra challenge, don't use:
//     // - recursion 递归
//     // Execute `rustlings hint iterators4` for hints.

//     // 递归
// }

// NOTE: 递归方法
pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        factorial(num - 1) * num
    }
}

// NOTE: (1..=num) 从 1 到 num
// NOTE: fold 任何形式的累计操作，都可以使用 fold
// 类似 JS 中的 reduce 函数
pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(0, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
