// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::ops::Add;

// NOTE: 不要将 trait 与其他语言的接口作类比
// 可以理解为标签。某个类型被打上了标签，就具有了标签拥有的功能
// 在特定的上下文中，帮助找到所要的类型的作用
trait AppendBar {
    fn append_bar(self) -> Self;
}

// 打上了标签，具有一个名字为 append_bar 的方法
// 方法的具体实现可以在此处实现
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    // 如果改成&mut，需要如何修改
    fn append_bar(&mut self) -> Self {
        self.add("Bar".into());
        self.to_owned()
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
