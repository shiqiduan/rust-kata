// 流程控制

// A. 条件语句
// A.1. if else
// A.2. match

// B. 循环
// B.1 loop, 是一个表达式，可以返回值
// B.2 while
// B.3 for
//      B.3.1
//          for item in collection -> 所有权转移
//          for item in &collection -> 不可变借用
//          for item in &mut collection -> 可变借用
//          如果元素实现了 copy，则直接复制，不会发生转移

// C. 退出和返回
// break、continue、return

fn demo() {
    let (a, b) = (10, 10);
    // 1. if else
    if a == b {
        println!("==");
    } else {
        println!("!=");
    }

    // 2. match
    let x = match a {
        10 => {
            2
        }
        _ => {
            1
        }
    };
    println!("{x}");
}

// B 循环
fn for_demo1() {
    // loop
    let mut a = 3;
    loop {
        if a < 0 {
            break;
        }
        a -= 1;
    }
    println!("Loop 退出了");

    // While
    let mut a = 3;
    while a > 0 {
        a -= 1;
    }
    println!("While 退出了");

    let a = [4,3,2,1];
    // 获取下标
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i, v);
    }

    for _ in 0..10 {
        println!("10");
    }
}

#[cfg(test)]
mod tests {
    use crate::basics::flow_control::{demo, for_demo1};

    #[test]
    fn test() {
        demo();
        for_demo1();
    }
}