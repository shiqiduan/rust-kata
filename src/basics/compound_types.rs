//! 复合类型

/// 元祖
pub fn tuple_demo() {
    // 创建
    let tup = (500, 10.0, "abc");
    // 解构
    let (x, y, z) = tup;
    println!("{x}{y}{z}");
    // 访问
    println!("{}", tup.2);
}

/// 结构体
/// 1. 定义结构体
pub fn struct_demo() {
    // 1. 定义结构体
    #[derive(Debug)]
    struct User {
        name: String,
        age: i32,
        active: bool,
    }

    // 初始化结构体
    let user = User {
        name: "abc".to_string(),
        age: 10,
        active: true,
    };

    // 访问结构体
    println!("{}", user.name);
    // 打印结构体 - #[derive(Debug)]
    // dbg!
    println!("{:?}", user);
    println!("{:#?}", user);

    // 结构体更新
    let user2 = User {
        name: "def".to_string(),
        // 必须在最后使用
        // 类似=，会发生所有权转移和复制
        ..user
    };

    // 元祖结构体. 关心名称而不是字段
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // 单元结构体. 关心行为而不是数据
    struct NoData;

    // 结构体数据的所有权
    // 如果是借用的数据，需要使用生命周期，确保结构体的作用范围比借用数据的范围要小
}

// 枚举 - 枚举类型、枚举值
/// 枚举 vs 结构体 ：可以使用同一的枚举类型
/// Option Result
fn enum_demo() {
    #[derive(Debug)]
    enum PokerSuit {
        Clubs(u8),
        Spades(u8),
        Diamonds(u8, char),
        Hearts { x: u8, y: char },
    }

    println!("{:?}", PokerSuit::Diamonds(10, 'A'));

    // 可以放到容器里面
    let vec = vec![
        PokerSuit::Clubs(10),
        PokerSuit::Spades(10),
        PokerSuit::Hearts { x: 5, y: 'a' },
    ];
    for x in vec {
        println!("{:?}", x);
    }
}

/// 数组 - array. 区别于 vector
fn array_demo() {
    // 创建
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 创建重复数组
    let a = [3; 5];
    // 访问数组
    println!(
        "{}, {}, {}",
        a[3],
        a.get(3).unwrap(),
        a.get(10).unwrap_or(&0)
    );
    // 复合类型
    let a: [String; 9] = std::array::from_fn(|_i| String::from("rust"));
    println!("{:#?}", a);

    // 切片
    let slice = &a[1..3];
    assert_eq!(slice, &["rust", "rust"]);
}

#[cfg(test)]
mod tests {
    use crate::basics::compound_types::{array_demo, enum_demo, struct_demo, tuple_demo};

    #[test]
    fn test_tuple() {
        tuple_demo();
    }

    #[test]
    fn test_struct() {
        struct_demo();
    }

    #[test]
    fn test_enum() {
        enum_demo();
    }

    #[test]
    fn test_array() {
        array_demo();
    }
}
