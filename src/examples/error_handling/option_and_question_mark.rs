#![allow(dead_code)]

/// 如果 x 是 Option，那么若 x 是 Some ，对x?表达式求值将返回底层值，否则无论函数是否正在执行都将终止且返回 None。
struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // 获取此人的工作电话号码的区号（如果存在的话）。
    fn work_phone_area_code(&self) -> Option<u8> {
        // 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句。
        self.job?.phone_number?.area_code
    }

    // 如果不使用？操作符，需要嵌套3层的match，且大部分都是重复的代码
    fn work_phone_area_code2(&self) -> Option<u8> {
        match self.job {
            Some(job) => match job.phone_number {
                Some(phone) => match phone.area_code {
                    Some(area_code) => return Some(area_code),
                    None => return None,
                },
                None => return None,
            },
            None => return None,
        }
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
    assert_eq!(p.work_phone_area_code2(), Some(61));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_and_question_mark() {
        main();
    }
}
