#![allow(dead_code)]

/// Rust 强制实行 RAII（Resource Acquisition Is Initiallization，资源获取即初始化），
/// 所以任何对象在离开作用域时，它的析构函数（destructor）就被调用，然后它占有的资源就被释放。

fn create_box(v: i32) {
    let _box = Box::new(v);
}

fn raii() {
    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里被销毁，内存得到释放
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for i in 0i32..1_000 {
        create_box(i);
    }
    // `_box2` 在这里被销毁，内存得到释放
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raii() {
        raii();
    }
}
