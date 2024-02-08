// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    let t = &word;
    // 问题：&String 为什么能够传递给 &str
    // String上有实现一个 derive  trait，在解引用的时候会自动做一个类型转换
    // 问题：什么时候发生的解引用?
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
