#[allow(dead_code)]
fn match_tuple() {
    let triple = (0, -2, 3);
    // 试一试 ^ 将不同的值赋给 `triple`

    println!("Tell me about {:?}", triple);
    // match 可以解构一个元组
    match triple {
        // 解构出第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (2, _, _) => println!("First is 2, and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        _ => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}

#[allow(dead_code)]
fn match_array() {
    // Try changing the values in the array, or make it a slice!
    let array = [7, -2, 6];

    match array {
        // 结构出第二个和第三个元素，绑定到变量上
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // 使用 _ 忽略某个元素
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 使用 .. 忽略剩余的元素
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 以下代码不会编译通过，解构时元素数量必须匹配，不想要的用..或_忽略
        // [-1, second] => ...

        // 可以解构时，将某些元素解构成子数组或者切片，类型需要对应（为数组时长度也要对应）
        // tail是[i32;1]数组类型
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // 甚至可以只取两端，将中间部分解构为子数组
        // middle是[i32;1]数组类型
        // 注意这个地方等于是default分支，匹配所有其他情况
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
fn match_enum() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

#[allow(dead_code)]
fn match_pointer() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let reference = &4;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

#[allow(dead_code)]
fn match_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // 结构体成员的顺序不重要，也可以给成员匹配新的名字，如x字段匹配后叫i
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 使用..忽略某些字段
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 下面的语句会报错，因为没有覆盖到x
        //Foo { y } => println!("y = {}", y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_tuple() {
        match_tuple();
    }

    #[test]
    fn test_match_array() {
        match_array();
    }

    #[test]
    fn test_match_enum() {
        match_enum();
    }

    #[test]
    fn test_match_pointer() {
        match_pointer();
    }

    #[test]
    fn test_match_struct() {
        match_struct();
    }
}
