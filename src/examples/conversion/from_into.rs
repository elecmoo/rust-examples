#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[allow(dead_code)]
fn from_into() {
    let num1 = Number::from(33);
    println!("Number1 is {:?}", num1);
    println!("Number1.value is {}", num1.value);

    let i = 22;

    // 如果为一个类型实现了From,那另外对应的那个类型将自动实现Into
    let num2: Number = i.into();

    println!("Number2 is {:?}", num2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frmo_into() {
        from_into();
    }
}
