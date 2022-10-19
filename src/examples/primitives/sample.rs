#[allow(dead_code)]
fn sample() {
    // 变量可以给出类型说明。
    let logical: bool = true;
    println!("logical: {}", logical);

    let a_float: f64 = 1.0; // 常规说明
    let an_integer = 5i32; // 后缀说明
    println!("a_float: {:.1}, an_integer: {}", a_float, an_integer);

    // 否则会按默认方式决定类型。
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    println!(
        "default_float: {:.N$}, default_integer: {}",
        default_float,
        default_integer,
        N = 3
    );

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 0; // 根据下下一行的赋值推断为 i64 类型
    println!("inferred_type: {}", inferred_type);
    inferred_type = 4294967296i64;
    inferred_type -= 1;
    println!("inferred_type: {}", inferred_type);

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 0;
    println!("mutable before: {}", mutable); // Mutable `i32`
    mutable = 21;
    println!("mutable after: {}", mutable);

    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
    println!("shadow mutable: {}", mutable);

    // 上面的变量都是定义了但是没有使用，编译器会进行警告，但是可以编译通过，通常建议解决这些警告
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        sample();
    }
}
