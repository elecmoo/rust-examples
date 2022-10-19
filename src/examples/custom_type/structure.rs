#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    // 嵌套解构
    let Rectangle {
        top_left: Point { x: a, y: b },
        bottom_right: Point { x: c, y: d },
    } = rect;
    (c - a) * (b - d)
}

fn square(point: Point, slide: f32) -> Rectangle {
    Rectangle {
        top_left: Point { ..point },
        bottom_right: Point {
            x: point.x + slide,
            y: point.y - slide,
        },
    }
}

#[allow(dead_code)]
fn structure() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", peter);
    println!("Perter.name: {}", peter.name);
    println!("Perter.age: {}", peter.age);

    // 实例化结构体 `Point`
    // 初始化时必须手动给每个值赋值，不像golang有默认值
    let point: Point = Point { x: 10.3, y: 0.4 };
    // 如果变量名和字段名一致，可以简写，不要key:
    let x = 3.3;
    let point2 = Point { x, y: 10.1 };
    // 可以使用一个已经初始化的实例来快速初始化另一个，使用..实例名这种语法，会把对应字段的赋值给新实例
    let point3: Point = Point { ..point };
    // 也可以将多种组合起来，用x初始化Point.x，其余的用旧实例point的对应字段初始化
    let point4 = Point { x, ..point };
    println!(
        "point: {:?}, point2: {:?}, point2: {:?}, point3: {:?}",
        point, point2, point3, point4
    );

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 15.2, y: 0.0 };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("area = {:.3}", rect_area(&rectangle));

    let p1 = Point { x: 3.0, y: 4.0 };
    let r = square(p1, 3.0);
    println!("area = {}", rect_area(&r));

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        structure();
    }
}
