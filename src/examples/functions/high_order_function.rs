#![allow(dead_code)]
fn is_odd(i: u32) -> bool {
    i % 2 == 1
}

// 高阶函数
fn high_order_function() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法，更加简洁，不过有时也更难理解，类似jave的Stream Api
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数取平方
        .take_while(|&n| n < upper) // 取小于上限的
        .filter(|&n| is_odd(n)) // 取奇数
        .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_order_function() {
        high_order_function();
    }
}
