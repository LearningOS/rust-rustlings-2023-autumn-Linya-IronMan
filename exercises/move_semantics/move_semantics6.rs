// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = 123; // 编译通过
    let a = "aaa".to_string(); // 编译错误

    get_char(a);

    print!("{}", a)
}

// Should not take ownership
fn get_char(data: String) -> char {}
