#![allow(dead_code)]

mod my {
    pub struct OpenBox<T> {
        // content是公共的，可以直接创建
        pub content: T,
    }

    pub struct CloseBox<T> {
        // content是私有的，需要构造方法才能初始化
        content: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(item: T) -> CloseBox<T> {
            CloseBox { content: item }
        }

        pub fn get_content(&self) -> &T {
            &self.content
        }
    }

    pub fn test1() {
        // 同一个模块中，不管共有私有，都能放访问到
        let b = CloseBox {
            content: "hello world",
        };
        println!("content of closebox: {}", b.content);
    }

    pub mod inner_my {
        pub fn test2() {
            // 内嵌的mod也能访问到父级的、祖先级的私有方法或属性
            let b = super::CloseBox {
                content: "hello world2",
            };
            println!("content of closebox: {}", b.content);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::my::*;

    #[test]
    fn test_pub_mod() {
        let b1 = OpenBox { content: 3 };
        println!("content in openbox: {}", b1.content);

        // 在mod tests中，无法观察到CloseBox中的私有属性，因为tests和my是同一级的模块，以下代码会报错
        // let b2 = CloseBox { content: 3 };

        // 可以使用共有的构造方法初始化
        let b2 = CloseBox::new(32);

        // 这里也一样，访问不了content，调用共有方法访问
        // println!("content in closebox: {}", b2.content);
        println!("content in closebox: {}", b2.get_content());
    }

    #[test]
    fn test_pub_mod2() {
        test1();
    }

    #[test]
    fn test_pub_mod3() {
        inner_my::test2();
    }
}
