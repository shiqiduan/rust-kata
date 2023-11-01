//! 生命周期 - 引用的有效作用域

/// 静态生命周期。‘static
/// 悬垂指针和生命周期
/// error[E0597]: `x` does not live long enough
fn lifetime_demo_01() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r:{}", r);
}

/// 函数中的生命周期 - why?
/// this function's return type contains a borrowed value,
/// but the signature does not say whether it is borrowed from `x` or `y`
/// "生命周期标注并不会改变任何引用的实际作用域“
fn lifetime_demo_02() {
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // 标注生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    // 证明生命周期实际取的是较小的那个
    // fn main() {
    //     let string1 = String::from("long string is long");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is {}", result);
    // }

    let s1 = String::from("abc");
    let s2 = "xyz123";
    let r = longest(&s1, s2);
    println!("{r}");
}

/// 结构体中的生命周期
fn lifetime_demo_03() {
    struct AAA<'a> {
        part: &'a str,
    }

    let s = String::from("abc");
    let a = AAA { part: &s };
}

/// 生命周期擦除
fn lifetime_demo_04() {
    // 返回值是引用类型，来自两个地方
    // 从参数获取
    // 从函数体内部新创建的变量获取 —— 悬垂指针，编译失败
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::lifetime::{lifetime_demo_01, lifetime_demo_02};

    #[test]
    fn test() {
        lifetime_demo_01();
        lifetime_demo_02();
    }
}
