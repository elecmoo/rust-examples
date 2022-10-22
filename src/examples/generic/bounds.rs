#![allow(dead_code)]

use std::fmt::{Debug, Display};

fn printer<T: Display>(t: &T) {
    println!("{}", t);
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// 可以使用多重约束，使用+号链接，表示多个trait都必须同时实现
fn my_print<T: Debug + Display>(t: &T) {
    println!("{}", t);
    println!("{}", t);
}

#[derive(Debug)]
struct Pointer(i32, i32);

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pointer<{}, {}>", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_bounds() {
        let pointer = Pointer(10, 20);

        printer(&pointer);

        print_debug(&pointer);

        my_print(&pointer);

        let p3 = Point3D(1, 2, 3);
        print_debug(&p3);
        // Point3D 没有实现Display，所以下面两个函数都无法调用
        // printer(&p3);
        // my_print(&p3);
    }
}
