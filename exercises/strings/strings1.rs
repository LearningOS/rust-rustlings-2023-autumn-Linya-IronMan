// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 字符串常量，存储在编译后的可执行程序代码段中, 一个静态常量的存储段中
    // 既不在堆上，也不再栈上
    // 可以理解为一个指针，指针指向的 blue 的实体，存在 .txt 段中 / 静态存储区中
    // "blue"
    // "blue".to_string()
    "blue".to_owned()
    // to_string to_owned 会将字符字面量复制一份存储到堆上
    // NOTE: to_string to_owned 区别
    // to_string
    // to_owned 将原本没有所有权的东西转变成有所有权的东西
}
