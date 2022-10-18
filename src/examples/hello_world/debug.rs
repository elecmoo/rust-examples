// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
fn my_debug() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[allow(dead_code)]
fn my_pretty_debug() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 普通的debug打印
    println!("{:?}", peter);

    println!("-----------------");

    // 美化打印，当然也没有美到哪里去
    println!("{:#?}", peter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_debug() {
        my_debug();
    }

    #[test]
    fn test_my_pretty_debug() {
        my_pretty_debug();
    }
}
