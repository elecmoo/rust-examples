#![allow(dead_code)]

use std::{fmt::Debug, io::Write};

struct BufBuild {
    buf: Vec<u8>,
}

impl BufBuild {
    // 构造函数
    pub fn new() -> Self {
        BufBuild {
            buf: Vec::with_capacity(1024),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        BufBuild {
            buf: Vec::with_capacity(cap),
        }
    }
}

impl Debug for BufBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuild {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        // 返回写入的数量
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // buf的flush啥也不用做
        Ok(())
    }
    // Write方法的接口的其他方法，trait提供了默认实现，不需要实现了
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buf() {
        let mut buf = BufBuild::new();

        let v: Vec<u8> = "hello world".into();

        if let Ok(r) = buf.write(&v) {
            assert_eq!(r, v.len());
        }

        println!("{:?}", buf);

        // 带有默认实现的方法也可以调用
        // 使用b前缀，可以直接将字符串字面量表示成Vec<u8>
        buf.write_all(b"!").unwrap();

        println!("{:?}", buf);
    }
}
