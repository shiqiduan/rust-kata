// 函数

// 要点
// 1. 函数名和变量名称使用蛇形命名法，例如：fn add_two(first_one: i32, second_one: bool)
// 2. 每个参数都必须标注类型
// 3. ! 作为函数返回类型的时候，表示该函数永不返回。
// 经典函数定义
fn add(i:i32, j:i32) -> i32 {
    i + j
}

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        // return 后面的分号可有可无
        return x - 5;
    }

    x + 5
}

#[cfg(test)]
mod tests {
    use crate::basics::function::{add, plus_or_minus};

    #[test]
    fn test() {
        let r = add(10, 20);
        assert_eq!(r, 30);
    }

    #[test]
    fn test_plus_or_minus() {
        assert_eq!(plus_or_minus(9), 4);
        assert_eq!(plus_or_minus(3), 8);
    }
}