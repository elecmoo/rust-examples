#![allow(dead_code)]

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(i32, Box<List>),
    // Nil: 链表末尾节点
    Nil,
}

impl List {
    // 创建一个空的链表实例
    fn new() -> List {
        List::Nil
    }

    // 在链表头部插入新元素，并返回该 List
    fn prepend(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。

        let mut n = 0_u32;
        let mut node = self;
        // 非递归写法
        loop {
            match *node {
                List::Nil => break,
                List::Cons(_, ref tail) => {
                    // 节点不为空，计数+1，节点后移一位
                    n += 1;
                    node = tail;
                }
            }
        }
        // 返回n
        n
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        let mut result = String::new();

        let mut node = self;
        loop {
            match *node {
                List::Nil => {
                    result.push_str("Nil");
                    break;
                }
                List::Cons(v, ref tail) => {
                    result.push_str(format!("{}, ", v).as_str());
                    node = tail;
                }
            }
        }
        result
    }
}

fn my_list() {
    // 创建一个空链表
    let mut list = List::new();
    println!("empty list length: {}", list.len());
    println!("empty list: {}", list.stringify());

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_list() {
        my_list();
    }
}
