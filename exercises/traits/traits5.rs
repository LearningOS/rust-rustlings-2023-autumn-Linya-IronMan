// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

mod AA {
    pub trait T1 {
        fn foo(&self) {}
    }

    impl T1 for String {}
}
pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// NOTE: 如果对 T 的限制较多，必须写在 where 上才可以
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {}
// NOTE: 孤儿原则
// trait 我们要实现的 trait，以及要实现 trait 的数据类型中，至少有一个是我们自己定义的
// impl error::Error for String {} error:Error 与 String 这两个都是标准库定义的，那么就不能这样实现
// 否则，任何人都可以修改任何类型A实现的trait 与 B 实现的 trait 如果有冲突，就会无法处理
// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T>(item: T) -> bool
where
    T: SomeTrait + OtherTrait,
{
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
