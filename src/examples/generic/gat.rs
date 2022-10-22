#![allow(dead_code)]

// 不适用泛型关联类型
mod no_gat {
    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
        fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
        fn last(&self) -> i32; // 未显式地要求 `A` 或 `B`
    }

    impl Contains<i32, i32> for Container {
        // 如果存储的数字和给定的相等则为真。
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // 得到第一个数字。
        fn first(&self) -> i32 {
            self.0
        }

        // 得到最后一个数字。
        fn last(&self) -> i32 {
            self.1
        }
    }

    // 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    pub fn no_gat_run() {
        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!(
            "Does container contain {} and {}: {}",
            &number_1,
            &number_2,
            container.contains(&number_1, &number_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }
}

// 使用泛型关联类型重构上面的例子
mod gat {
    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains {
        // 在这里定义可以被方法使用的泛型类型。
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
        // 为 `Container(i32, i32)`，那么 `output`（输出）类型
        // 会被确定为 `i32` 和 `i32`。
        type A = i32;
        type B = i32;

        // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // 得到第一个数字。
        fn first(&self) -> i32 {
            self.0
        }

        // 得到最后一个数字。
        fn last(&self) -> i32 {
            self.1
        }
    }

    // 这里就不再需要A，B两个类型了
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    pub fn gat_run() {
        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!(
            "Does container contain {} and {}: {}",
            &number_1,
            &number_2,
            container.contains(&number_1, &number_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_generic() {
        no_gat::no_gat_run();
    }

    #[test]
    fn test_gat() {
        gat::gat_run();
    }
}
