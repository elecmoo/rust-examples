// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::fmt;

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Structure(i32);

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "**{}**", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag > 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

#[allow(dead_code)]
fn display() {
    let s = Structure(88);
    // 定义了disploy就可以使用“{}”来打印了
    println!("{}", s);

    let c = Complex {
        real: 10.0,
        imag: -3.3,
    };

    // debug
    println!("{:?}", c);
    // display
    println!("{}", c);

    let c1 = Complex {
        real: 3.0,
        imag: 4.0,
    };
    println!("{}", c1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disploy() {
        display();
    }
}
