#[allow(dead_code)]
fn for_in() {
    // 1..101生成[1, 101)的区间，左包含右不包含
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 1..=3生成[1, 3],两端均包含
    for n in 1..=3 {
        println!("{}", n);
    }
}

#[allow(dead_code)]
fn for_iter() {
    let names = vec!["Bob", "Frank", "Tom"];

    // iter在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
    // 注意这里取出来的元素name的类型是&&str类型，是集合元素&str的引用类型
    for name in names.iter() {
        match name {
            &"Frank" => println!("This is Frank"),
            _ => println!("Hello {}", name), // 匹配其余的，类似default
        }
    }

    println!("names: {:?}", names);
}

#[allow(dead_code)]
fn for_into_iter() {
    let names = vec!["Bob", "Frank", "Tom"];

    // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。
    // 一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 “移除”（move）了。
    // 注意这里取出来的name是&str类型，即元素的原始类型
    for name in names.into_iter() {
        match name {
            "Frank" => println!("This is Frank"),
            _ => println!("Hello {}", name), // 匹配其余的，类似default
        }
    }

    // 注意，集合names已经被消耗，无法再使用，下面的代码无法编译通过
    // println!("names[1] = {}", names[1]);
}

#[allow(dead_code)]
fn for_iter_mut() {
    // 注意，要想使用iter_mut，names必须要可以修改
    let mut names = vec!["Bob", "Frank", "Tom"];

    // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
    // 集合不会被消耗，迭代完成后仍然可以使用
    // 注意name类型为&mut &str，为集合元素的可变引用类型
    for name in names.iter_mut() {
        *name = match name {
            &mut "Frank" => "This is Frank",
            _ => "xxx", // 匹配其余的，类似default
        }
    }
    println!("names: {:?}", names);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_in() {
        for_in();
    }

    #[test]
    fn test_for_iter() {
        for_iter();
    }

    #[test]
    fn test_for_into_iter() {
        for_into_iter();
    }

    #[test]
    fn test_for_mut() {
        for_iter_mut();
    }
}
