// 闭包可以作为返回值返回，然而返回闭包类型会有问题，
// 因为目前 Rust 只支持返回具体（非泛型）的类型。
// 按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包，指明比表是Fn、FnMut、FnOnce中的具体那个。
// 除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过值进行的。这是必须的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。

#![allow(dead_code)]

// 这个impl关键字是必须的，函数需要返回一个具体的类型，不能是泛型的
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    // 作为返回参数时，闭包必须强制按照移动变量进行捕获
    // 不过这样有个疑问，那个定义的Fn，FnMut, FnOnce还有啥意义呢，不都是FnOnece了吗？困惑
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn closure_as_return() {
    let fn_plain = create_fn();

    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_as_return() {
        closure_as_return();
    }
}
