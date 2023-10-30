// 基本类型是语言内置类型
// 1. 布尔类型 - bool - true or false
// 2. 数值类型
//      2.1 整数类型
//         2.1.1 有符号整数 - i8、i16、i32、i64、isize
//         2.1.2 无符号整数 - u8、u16、u32、u64、usize
//      2.2 浮点数类型 - f32、f64
//
//      数字字面量
//          十进制 = 98_222
//          十六进制 = 0xff
//          八进制 = 0o78
//          二进制 = Ob1111_0000
//          字节(u8) = b'A'
//
//
// 3. 字符类型 - ‘’ - 一个 unicode 编码即为字符
// 4. 元组类型
// 5. 数组类型
// 6. 切片类型
// 7. 单元类型 Unit Type，单元值 or 单元字面量 - (), 充当占位符

// A. 其他概念和要点
// A.1 类型推导和标注 ?
// A.2 整数溢出
//      A.2.1 debug 模式下，溢出会发生 panic.强校验
//      A.2.2 release 模式下，会按照补码循环溢出规则处理，不会 panic. u8的情况下，256 = 0，257 = 1。
//      A.2.3 溢出可以显式处理
// A.3 位运算
// A.4 序列
// A.5 类型转换 as

pub fn demo() {
    let a:u8 = 255;
    // let b = a + 1;
    // println!("{a}, {b}");

    let c = a.wrapping_add(20);
    println!("{a}, {c}");

    // 2.2 浮点数, panic
    // assert!(0.1 + 0.2 == 0.3);

    // 3. 字符类型
    let x = '中';
    println!("{x}");
    println!("size of bytes: {}", std::mem::size_of_val(&x));

    // A.4 序列
    for i in 1..= 5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::primitive_types::demo;

    #[test]
    fn test() {
        demo();
    }
}