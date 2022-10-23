#![allow(dead_code)]

/// 所有权规则
/// 1. Rust 中的每一个值都有一个 所有者（owner）。
/// 2. 值在任一时刻有且只有一个所有者。
/// 3. 当所有者（变量）离开作用域，这个值将被丢弃。

// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` 被销毁且内存得到释放
}

fn ownership_move() {
    // 栈分配的整型
    // 基本类型基本都是copy的，其分配在栈上，赋值时会复制一个值的副本，原变量仍然具有所有权
    let x = 5u32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    // 堆上值的生命周期通常和其栈上的变量保持一致，变量离开作用域时，值被drop
    // 赋值时所有权会发生move，原变量将无法使用
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    // 报错！和前面出错的原因一样。
    //println!("b contains: {}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_move() {
        ownership_move();
    }
}
