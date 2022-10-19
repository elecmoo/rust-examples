#[allow(dead_code)]
fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    // if形成的块也是表达式，有值，可以赋值给变量
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 这个表达式返回一个 `i32` 类型。
        10 * n
    } else {
        println!(", and is a big number, half the number");

        // 这个表达式也必须返回一个 `i32` 类型。
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_else() {
        if_else();
    }
}
