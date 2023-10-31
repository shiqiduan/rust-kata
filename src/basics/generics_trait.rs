// 泛型

// 好处：可以提供代码的重用性和灵活性，如何实现呢？
// 多态 vs 泛型 = 动态 vs 静态多态（类型擦除）
// 泛型的实现 - 零成本抽象，静态编译实现，单态化实现
// 泛型的 tradeoff - 无性能损失。编译速度和文件大小

// B. Trait

// 孤儿规则
// b.2 使用特征作为函数参数
// b.3 Trait bound
// b.4 多重约束
// b.5 where 约束
// b.6 使用特征约束有条件地实现方法或特征
// b.7 函数返回中的 impl trait
// b.8 通过 derive 派生特征
// b.9 特征对象

// 对比 rust 和 java 的泛型和接口的实现

use std::fmt::{Debug, Display};

// 特定类型的函数
fn largest_i32(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &x in list {
        if x > max {
            max = x;
        }
    }
    max
}

// 经典案例
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &item in list.iter() {
        if item > max {
            max = item
        }
    }
    max
}

// 方法中使用泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// const 泛型
// [i32;2] 与 [i32;3] 是不同的泛型
// N 就是常量泛型，语法：const N: usize
fn demo_01() {
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

///////////
// Trait

pub trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    fn default_impl(&self) -> String {
        println!("hello");
        String::from("abc")
    }
}

// b.2 使用特征作为函数参数
pub fn notify(item: &impl Summary) {
    println!("")
}

// b.3 特征约束
pub fn nofity<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// b.4 多重约束
pub fn notify_01(item: &(impl Summary + Display)) {}

pub fn notify_02<T: Summary + Display>(item: &T) {}

// b.5 where 约束
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}
// b.6 使用特征约束有条件地实现方法或特征
// b.7 函数返回中的 impl trait

// b.9 特征对象
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area(shape: impl Shape) {
    println!("Area: {}", shape.area());
}

#[cfg(test)]
mod tests {
    use crate::basics::generics_trait::{
        demo_01, largest, largest_i32, print_area, Circle, Point, Rectangle, Shape,
    };

    #[test]
    fn test() {
        let m = largest_i32(&vec![1, 2, 3, 4, 5]);
        println!("{m}");

        let m = largest(&vec![1, 2, 3, 4, 5]);
        println!("{m}");
    }

    #[test]
    fn test_point() {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());

        let p = Point { x: "a", y: "b" };
        println!("p.x = {}", p.x());
    }

    #[test]
    fn test_demo_01() {
        demo_01();
    }

    #[test]
    fn test_print_area() {
        let rectangle = Rectangle {
            width: 5.0,
            height: 3.0,
        };

        let circle = Circle { radius: 2.5 };
    }
}
