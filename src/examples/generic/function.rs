#![allow(dead_code)]

struct GenericVal<T>(T);

impl<T> GenericVal<T> {
    fn foo(&self) {}
}

fn show_val<T>(_v: &GenericVal<T>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        let v = &GenericVal("hello world");

        show_val(v);

        v.foo();
    }
}
