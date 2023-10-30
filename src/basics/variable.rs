// 这是完整的吗？

// 1. 为什么要区分可变变量和不可变变量？更精细的编译器控制粒度
// 2. 变量绑定 - 为什么不用赋值而用绑定呢，binding？涉及到所有权的概念
// 3. 变量可变性
//      3.1 重复赋值的异常：error[E0384]: cannot assign twice to immutable variable `a`
// 4. 未使用的变量可以通过下划线前缀压制警告信息
// 5. 变量解构
// 6. 常量。constant，与变量的区别
//      6.1 关键字是 const，并且只的类型必须标注。
//      6.2 在编译期完成计算
// 7. 变量遮蔽（shadowing）
//      why - 在某一个作用域内，复用一个变量名称.
//      7.1 与 mut 的区别是，let 重新生成了一个同名的新变量，涉及到内存的重新分配。

pub fn demo() {
    // 2. 变量绑定
    let a = "hello world";
    // 3.1
    // a = "aaa";

    // 3. 变量可变性
    let mut b = 123;
    b = b + 1;
    b += 1;

    // 4. 未使用变量。warning: unused variable: `x`
    // let x = 100;
    // 4.1 忽略返回值。let _ = add();
    let _x = 100;

    println!("{a}, {b}");

    // 5. 变量解构。
    // a = true, 不可变; b = false, 可变
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
}

// 解构式赋值 ? 参考模式匹配
struct Struct {
    e: i32
}

// 6. 常量
const A_B_C:i32 = 100;

// 7. 变量遮蔽（shadowing）
fn shadowing_demo() {
    let x = 5;
    // 相同作用域内进行遮蔽
    let x = x + 1;

    {
        // 子作用域内进行遮蔽。
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    // 重新绑定，忽略之前的信息和类型
    let x = "hello world";
    println!("The value of x is: {}", x);
}

#[cfg(test)]
mod tests {
    use crate::basics::variable::{demo, shadowing_demo};

    #[test]
    fn test() {
        let _ = demo();
        // assert_eq!(result, 5);

        shadowing_demo();
    }
}