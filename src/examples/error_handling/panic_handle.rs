#![allow(dead_code)]

fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 一些函数应当在特定条件下 panic。为测试这种行为，请使用 #[should_panic] 属性。
    /// 这个属性接受可选参数 expected = 以指定 panic 时的消息。如果你的函数能以多种方式 panic，这个属性就保证了你在测试的确实是所指定的 panic。
    #[test]
    // #[should_panic]
    #[should_panic(expected = "AAAaaaaa!!!!")]
    fn test_panic() {
        main();
    }
}
