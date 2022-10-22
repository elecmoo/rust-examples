#![allow(dead_code)]

// 不可复制的类型。
struct Empty;
struct Null;

// `T` 的泛型 trait。
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
    // 注意这里方法需要接受两个值的所有权，会导致值得所有权移动到函数内部，函数
    // 调用完成后，值被释放
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放它们。
    fn double_drop(self, _: T) {}
}

fn generic_traits() {
    let empty = Empty;
    let null = Null;

    // 释放 `empty` 和 `null`。
    empty.double_drop(null);

    // empty;
    // null;
    // ^ 试一试：去掉这两行的注释。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_traits() {
        generic_traits();
    }
}
