#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[allow(dead_code)]
fn try_from_and_try_into() {
    // TryFrom 尝试转化，会生成Result，可能报错
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryFrom和TryInto是一对，为一个类型实现了TryFrom，另一个对应类型将自动实现TryInto
    let result: Result<EvenNumber, ()> = 8.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5.try_into();
    assert_eq!(result, Err(()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_and_try_into() {
        try_from_and_try_into();
    }
}
