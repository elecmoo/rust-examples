#![allow(dead_code)]

fn age() -> u32 {
    15
}

fn match_bind_variables() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但是具体是几无法获取到
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn match_bind_enum() {
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        // 也可以使用 n @ 10..=50中间n绑定到一个值序列中的某一个
        Some(n @ 10..=50) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n) => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_bind_variables() {
        match_bind_variables();
    }

    #[test]
    fn test_match_bind_enum() {
        match_bind_enum();
    }
}
