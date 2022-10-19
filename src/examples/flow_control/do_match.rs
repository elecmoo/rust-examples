#[allow(dead_code)]
fn do_match() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 试一试 ^ 将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        // 所有情况都必须覆盖到，否则会报错
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_match() {
        do_match();
    }
}
