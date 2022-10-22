#![allow(dead_code)]

// 虚类型（phantom type）参数是一种在运行时不出现，而在（且仅在）编译时进行静态检查 的类型参数。
// 可以用额外的泛型类型参数指定数据类型，该类型可以充当标记，
// 也可以供编译时类型检查 使用。这些额外的参数没有存储值，也没有运行时行为。

use std::marker::PhantomData;
use std::ops::Add;

/// 创建空枚举类型来表示单位。
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `Length` 是一个带有虚类型参数 `Unit` 的类型，
/// 而且对于表示长度的类型（即 `f64`）而言，`Length` 不是泛型的。
///
/// `f64` 已经实现了 `Clone` 和 `Copy` trait.
/// 第一个值是长度的值，第二个虚类型是长度的单位
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// `Add` trait 定义了 `+` 运算符的行为。
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // add() 返回一个含有和的新的 `Length` 结构体。
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        // `+` 调用了针对 `f64` 类型的 `Add` 实现。
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn phantom_type() {
    // 实际上第二个参数可以理解为，只要标记下就行，PhantomData纯粹是为了占个位置
    // 指定 `one_foot` 拥有虚类型参数 `Inch`。
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` 拥有虚类型参数 `Mm`。
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` 调用了我们对 `Length<Unit>` 实现的 `add()` 方法。
    //
    // 由于 `Length` 了实现了 `Copy`，`add()` 不会消耗 `one_foot`
    // 和 `one_meter`，而是复制它们作为 `self` 和 `rhs`。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行。
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 无意义的运算当然会失败：
    // 编译期错误：类型不匹配。
    //let one_feter = one_foot + one_meter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phantom_type() {
        phantom_type()
    }
}
