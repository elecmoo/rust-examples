#[allow(dead_code)]
fn mutable() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    // _immutable_binding += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable() {
        mutable();
    }
}
