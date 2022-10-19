#![allow(unreachable_code)]

#[allow(dead_code)]
fn my_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}

#[allow(dead_code)]
fn loop_and_lable() {
    'outer: loop {
        println!("Entered the outer loop");

        loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

#[allow(dead_code)]
fn break_with_value() {
    let mut counter = 0;

    // loop表达式也可以带值，放在break 语句之后，注意这里要加分号
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // loop表达式的值，需要分号
        }
    };

    assert_eq!(result, 20);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_loop() {
        my_loop();
        loop_and_lable();
        break_with_value();
    }
}
