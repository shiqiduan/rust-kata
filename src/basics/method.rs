// 方法

// rust 的对象，数据和方法分离有什么好处？ 可以扩展吗？
// 1. 方法的定义 - impl

// 2. self 参数：
//
// self 参数表示当前实例的引用（&self），可变引用（&mut self），或所有权（self）。
// &self：用于只读访问实例的方法。
// &mut self：用于可变地修改实例的方法。
// self：用于获取实例所有权并进行转移。

// 3. 关联函数
// 没有 self 参数，类似其他语言的静态方法和类方法

// 4. 方法调用

// 5. 自动引用和解引用
// 6. 链式调用

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // 3. 关联函数
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn demo() {
    // 4. 关联方法调用，命名空间
    let r = Rectangle::new(100, 3);
    // 4. 方法调用
    println!("area = {}", r.area());
}

#[cfg(test)]
mod tests {
    use crate::basics::method::demo;

    #[test]
    fn test() {
        demo()
    }
}
