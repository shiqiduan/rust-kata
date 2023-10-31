// 所有权和借用

// Why: 所有权是不需要 GC 解决内存安全性的解决方案
// 重要的一点：它是编译器的静态检查规则，运行时依然需要并发控制
//
// 内存管理 - 堆 & 栈 - 内存安全问题 - 是否完整？
// （无效引用）悬空指针（Dangling Pointer）- 指已释放的内存指针
// （无效引用）野指针 - 指向已释放或无效内存的指针
// 数据竞争 - 静态规则 + 并发控制
// 多重释放 -> 无效引用

// 所有权原则 or 模型
// 1. rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 2.一个值同时只能被一个变量所持有，或者说一个值只能有一个所有者
// 3. 当所有者离开作用域范围时，这个值才会被丢弃（drop）
// 4. 所有权可以通过绑定、函数参数和返回值进行传递
// 5. 当所有权转移时，之前的所有者将无法使用数据
// 相关概念：值、变量、所有者、作用域、drop、move

// 具体应用
// 1. 所有权转移 - copy、move、borrow
// 2. 引用与借用 - 灵活性
//      2.1 引用 - 所谓引用就是指针，&操作，*操作
//      2.2 借用 - 不可以引用 & 可变引用 - 类比：读写锁，但是有较大区别

//          编译器避免数据竞争有哪些好处？最大的是安全性
//          具体规则 = borrow checker
//          借用规则、生命周期和所有权模型

// 1. 所有权转移 - 变量绑定
fn demo() {
    let x = 5;
    // 基本类型的 copy
    let y = x;
    println!("{x}{y}");

    // 字面量放在常量区，常量区由编译器和运行时管理，不需要所有权。
    // 所有权是用来管理堆上的数据的。
    // x 持有常量数据的共享引用，只读
    let x = "abc";
    // 指针发生复制
    let y = x;
    println!("{x}{y}");

    // x 是 String 类型，没有实现 copy 的 trait.
    let x = String::from("abc");
    // value moved here
    // 语言实现有两种选择：1, copy堆上的数据，性能会受影响，2，copy 引用地址，但是违反了所有权规则。
    // 执行 move，完成栈上的数据复制，同时完成所有权转移
    let y = x;
    // error[E0382]: borrow of moved value: `x`
    // println!("{x}{y}");

    let x = String::from("abc");
    // 克隆，深度拷贝，两份数据，两个所有者
    let y = x.clone();
    println!("{x}{y}");
}

// 函数的传值与返回
// 参数和返回值的处理，也会涉及到 copy、move、clone 相关的概念
fn demo_01() {}

#[cfg(test)]
mod tests {
    use crate::basics::ownership_borrowing::demo;

    #[test]
    fn test() {
        demo();
    }
}
