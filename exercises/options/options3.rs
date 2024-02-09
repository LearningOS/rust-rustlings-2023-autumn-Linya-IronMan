// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // NOTE: 不想再 match 的时候发生所有权的转移, 在 match 之后，就无法再使用 y 变量了
    // 使用 ref p的类型就会推导为 &Point
    match y {
        // Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    match y {
        Some(Point { x, ref y }) => println!("Co-ordinates are {} {}", x, y),
        _ => panic!("no match!"),
    }

    match &y {
        Some(Point { x, y }) => println!("Co-ordinates are {} {}", x, y),
        _ => panic!("no match!"),
    }

    y; // Fix without deleting this line.
}
