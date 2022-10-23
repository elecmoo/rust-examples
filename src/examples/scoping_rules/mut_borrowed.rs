#![allow(dead_code)]

/// 可变数据可以使用 &mut T 进行可变借用。这叫做可变引用（mutable reference），它使借用者可以读/写数据。
/// 相反，&T 通过不可变引用（immutable reference）来借用数据，借用者可以读数据而不能更改数据
/// 引用规则：
/// - 在任意给定时间，要么 只能有一个活跃的可变引用，要么 只能有多个不可变引用。即可变引用与可变应用以及不可变引用互斥，多个不可变引用可以共存
/// - 引用必须总是有效的。
/// 注意，这里说的"活跃的"可变引用指的是真正被用于改变值的可变引用，如果把可变引用当不可变引用使用，算不可变的
///

struct Book {
    // `&'static str` 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对 Book 类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn mut_borrowed() {
    // 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 报错！不能可变地借用一个不可变对象
    // new_edition(&mut immutabook);

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);

    // 可变地借用一个可变对象
    new_edition(&mut mutabook);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn mut_borrowed2() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    // let mutable_borrow = &mut point;

    // 被借用的值在这里被重新使用
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
    // let y = &point.y;

    // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);

    // 正常运行！可变引用能够以不可变类型传入 `println!`
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // 可变引用不再用于其余的代码，因此可以重新借用
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_borrowed() {
        mut_borrowed();
    }

    #[test]
    fn test_mut_borrowed2() {
        mut_borrowed2();
    }
}
