// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // NOTE：Vec<_> 只关心外层是一个 Vector 即可，内部类型不关心
    // 也可以什么类型都不写，让编译器自动进行类型推导
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
