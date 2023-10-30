// 模式匹配

// 模式
// 1. match
//      1.1 匹配的顺序，第一个成功即返回
//      1.2 match必须穷举所有可能， _ =>
//      1.2 运行解构和绑定
//      1.1 模式的种类. 待细化
//          1.1.1 字面量
//          1.1.2 变量
//          1.1.3 通配符
//          1.1.4 解构的数组、枚举、结构体或者元组
// 2. if let，适用于只有一个模式需要被处理的情况
// 3. Option

enum Direction {
    East,
    West,
    North,
    South,
}

fn demo() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    }
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn demo_01() {
    let actions = [
        Action::Say("Hello world".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 0, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{s}");
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

// 1.1 模式的种类. 待细化
fn pattern_demo() {
    // 字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 单分支多模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过序列..=匹配值的范围
    let x = 5;
    match x {
        1..=5 => println!("one to five"),
        _ => println!("something else"),
    }

    // 解构 - 结构体、枚举、元组、数组和引用
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 10 };
    match p {
        Point { x, y: 0 } => println!("y = 0"),
        Point { x: 0, y } => println!("x = 0"),
        Point { x, y } => println!("point"),
    }

    // 用..忽略剩余值
    let point = Point { x: 0, y: 0 };
    match point {
        Point { x, .. } => println!("point"),
    }

    // 匹配守卫提供的额外条件
    let x = Some(4);

    match x {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // @绑定，绑定到新的变量
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

// 2. if let
fn demo_02() {
    // 使用 match 处理
    let a = Some(100);
    match a {
        Some(x) => println!("{x}"),
        _ => (),
    }

    // if let
    // 变量遮蔽
    if let Some(x) = a {
        println!("{x}");
    }
}

// 3. Option
fn option_demo() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

#[cfg(test)]
mod tests {
    use crate::basics::pattern_match::{demo, demo_01, demo_02, option_demo, pattern_demo};

    #[test]
    fn test() {
        demo();
        demo_01();
        demo_02();
        option_demo();
        pattern_demo();
    }
}
