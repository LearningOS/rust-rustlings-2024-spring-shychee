// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.
// 步骤：
// 1.如果提供的字符串长度为0，则应返回错误
// 2.将给定的字符串拆分为其中的逗号
// 3.拆分后只应返回2个元素，否则返回错误
// 4.从拆分操作中提取第一个元素并将其用作名称
// 5.从split操作中提取另一个元素，并将其解析为“usize”作为age，类似于“4”。parse:：<usize>（）`
// 6.如果在提取姓名和年龄时出现问题，则应返回错误
// 如果一切顺利，则返回Person对象的Result
// 顺便说一句：`Box<dyn Error>`实现了`From<&'_str>`。这意味着，如果你想返回一个字符串错误消息，你可以通过使用return `Err（“my error message”.into（））`来实现。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // steps 1
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }
        // steps 2
        // steps 3
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() !=2 {
            return Err(ParsePersonError::BadLen);
        }
        // steps 4
        let name = parts[0];
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        // steps 5
        let age_str = parts[1];
        match age_str.parse::<usize>() {
            Ok(age) => Ok(Person {
                name: name.to_string(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e))
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
