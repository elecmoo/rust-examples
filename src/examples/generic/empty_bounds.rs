// 约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，
// 你仍然可以用它 作为约束。标准库中的 Eq 和 Ord 就是这样的 trait。

#![allow(dead_code)]

struct A;
struct B;

// 这是一个空trait，但是仍然是有意义的
trait Empty {}

// 为A实现空trait
impl Empty for A {}

fn need_empty<T: Empty>(_: &T) {}

fn empty_bounds() {
    let a = A {};

    let _b = B {};

    need_empty(&a);
    // B没有实现Empty，不能作为参数传递给need_empty
    // need_empty(&_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_bounds() {
        empty_bounds();
    }
}
