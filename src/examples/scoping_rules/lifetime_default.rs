#![allow(dead_code)]

/// 有些生命周期的模式太常用了，所以借用检查器将会隐式地添加它们以减少程序输入量和增强可读性。
/// 编译器的默认生命周期推导规则如下：
/// -  所有引用类型的参数都有独立的生命周期 'a 、'b 等。
/// - 如果只有一个引用型输入，它的生命周期会赋给所有输出。
/// - 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。

// `elided_input` 和 `annotated_input` 事实上拥有相同的签名，
// `elided_input` 的生命周期会被编译器自动添加：
fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x)
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x)
}

// 类似地，`elided_pass` 和 `annotated_pass` 也拥有相同的签名，
// 生命周期会被隐式地添加进 `elided_pass`：
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn lifetime_default() {
    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_default() {
        lifetime_default();
    }
}
