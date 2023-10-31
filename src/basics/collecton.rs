// 集合类型：Vector and HashMap

// Vector - 只能存储相同类型的值。不同类型的可以使用枚举或者特征对象？

use std::collections::HashMap;

fn vector_demo() {
    // 创建方式 1
    // let mut v = vec![1, 2, 3];
    // 创建方式 2
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for x in v.iter() {
        println!("{x}");
    }
    println!("{}", v.capacity());

    // 获取元素
    let x = &v[1];
    println!("{x}");
    match v.get(1) {
        Some(a) => println!("{a}"),
        None => println!("no"),
    }
}

fn vector_demo_01() {
    // 同时访问多个数组元素
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // println!("{first}");
}

fn vector_demo_02() {
    // 存储不同类型的元素：枚举
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V5(String),
    }

    let v = vec![IpAddr::V4("127".to_string()), IpAddr::V5("111".to_string())];

    for ip in v {
        println!("{:?}", ip);
    }
}

fn vector_demo_03() {
    // 存储不同类型的元素：特征对象
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0);
        }
    }
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0);
        }
    }

    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("111".to_string())),
        Box::new(V6("666".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

fn vector_sort() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort_unstable();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

// HashMap

fn hashmap_demo_01() {
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // get -> Option
    let a = map.get("a");
    println!("{:?}", a);
    // get
    let d = map.get("d").copied().unwrap_or(0);
    // 查询对应的值，如果不存在则插入新的值
    let e = map.entry("e").or_insert(5);
    println!("{e}");
    // 查询对应的值，如果不存在则插入新的值。已经存在，不变
    let e = map.entry("e").or_insert(500);
    assert_eq!(*e, 5);
    // 查询对应的值，如果不存在则插入新的值. 更新
    let e = map.entry("e").or_insert(500);
    *e += 1;
    assert_eq!(*e, 6);
    let e = map.get("e").copied().unwrap_or(0);
    assert_eq!(e, 6);
}

fn hashmap_demo_02() {
    // vec 转 map
    let vec = vec![("a", 100), ("b", 200), ("c", 300)];
    let map: HashMap<_, _> = vec.into_iter().collect();
    println!("{:?}", map);
}

#[cfg(test)]
mod tests {
    use crate::basics::collecton::{
        hashmap_demo_01, hashmap_demo_02, vector_demo, vector_demo_01, vector_demo_02,
        vector_demo_03, vector_sort,
    };

    #[test]
    fn test() {
        vector_demo();
    }

    #[test]
    fn test_vector_demo_01() {
        vector_demo_01();
    }

    #[test]
    fn test_vector_demo_02() {
        vector_demo_02();
    }

    #[test]
    fn test_vector_demo_03() {
        vector_demo_03();
    }

    #[test]
    fn test_vector_sort() {
        vector_sort();
    }

    #[test]
    fn test_hashmap() {
        hashmap_demo_01();
        hashmap_demo_02();
    }
}
