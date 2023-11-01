// 返回值和错误处理

// 和 Java 比较的区别是什么呢？
// 1. 没有区分可检查异常和不可检查异常。强制必须处理错误。
// 2. 没有异常的对象概念，统一是错误处理，性能更好

// 问题：我如何知道我要处理的是哪些异常呢？我如何通知我的上游？
// 通过Result 里面的 err 类型的定义，可以自定义错误信息
// from 特征是什么？为了做类型转换。

use std::fs::File;
use std::io;
use std::io::Read;

fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

// ? 结合链式调用
fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
// cannot use the `?` operator in a function that returns `()`
fn error_demo() {
    // let f = File::open("hello.txt")?;
}

// from 特征

struct MyType {
    value: i32,
}

impl From<i32> for MyType {
    fn from(value: i32) -> Self {
        MyType { value }
    }
}

fn from_demo() {
    let my_value: MyType = 42.into();
    println!("value: {}", my_value.value);
}

#[cfg(test)]
mod tests {
    use crate::basics::return_panic::{error_demo, from_demo};

    #[test]
    fn test() {
        error_demo();
        from_demo();
    }
}
