// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// NOTE: 返回值的声明周期，不能比 x y 的生命周期长。输入的生命周期不能比输出的生命周期还要长
// 这是为了安全的使用返回值考虑的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 只要 b 还存活，a 一定也要存活。
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
