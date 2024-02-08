// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    red: u8,
    green: u8,
    blue: u8,
}

// 元组类型的 struct
// 实际使用的时候，可以看成一个函数去使用
// 在做 map 操作的时候，可以将这个 struct 的名字传进去
// 这个 struct 的名字本质上是一个构造函数
struct ColorTupleStruct(u8, u8, u8 /* TODO: Something goes here */);
// .0 .1 .2 来进行访问

// 派生宏 —— 过程宏的一个分类, 为 struct 增加额外的功能。给 struct 添加一些方法，实现一些 trait
// 想要通过 ?; 打印出此结构体的调试信息的形式，就需要派生 debug
// #derive(Copy) bit 级别的拷贝后没有任何副作用
// 派生 Copy 之后，a = b，会进行 Copy，而不是 move
// 要求这个 struct 中的所有成员都是可以 Copy 的
#[derive(Debug)]
struct UnitLikeStruct;
// 单元 struct ；ZST 0size 类型 ，不占有内存空间
// 可以标记一个位置？

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
