#![allow(dead_code)]

fn move_and_mut() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

// 变量可以被部分move
fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);

    // 这里仍然可以对person继续解构获取age，注意name用的_
    // 由于age是通用类型，实现了copy语义，不会move，只会复制一个变量副本
    // 因此，即使不用引用，解构后person.age依然可以使用，但persion.name不行
    let Person { name: _, age: age2 } = person;
    println!("The person's age is {}", age2);
    println!("The person's age from person struct is {}", person.age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_and_mut() {
        move_and_mut();
    }

    #[test]
    fn test_partial_move() {
        partial_move();
    }
}
