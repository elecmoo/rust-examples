#![allow(dead_code)]

// 该函数将闭包作为参数并调用它。
// 注意，闭包作为函数参数是，必须要是泛型类型，因为在闭包捕获自由变量时，编译器会隐式地创建一个匿名类型的结构体，用以储存闭包捕获的变量，
// 同时为这个未知类型的结构体实现函数功能，通过 Fn、FnMut 或 FnOnce 三种 trait 中的一种。
// 若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的。
// 然而，使用未限定类型的参数 <T> 过于不明确，并且是不允许的。
// 事实上，指明为该结构体实现的是 Fn、FnMut、或 FnOnce 中的哪种 trait，对于约束该结构体的类型而言就已经足够了。
// 因此下面的泛型只是指明f的具体类型，而不惯性这个闭包会不会那种类型的参数
fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32
where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32,
{
    f(3)
}

// 例如用一个类型说明为 FnOnce 的闭包作为参数。
// 这说明闭包可能采取 &T，&mut T 或 T 中的一种捕获方式，但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。
// 这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。注意反过来就不再成立：如果参数的类型说明是 Fn，那么不允许该闭包通过 &mut T 或 T 捕获变量。
fn closure_as_params() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };
    // diary闭包需要的最严的是FnOnce，因此，其类型就是FnOnce

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_as_param() {
        closure_as_params();
    }
}
