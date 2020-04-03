use std::convert::{From, TryFrom, TryInto};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn for_from() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    //into就是from反过来，实现了from就免费获得into
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

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

pub fn for_tryfrom() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    //    TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

pub fn for_tostring() {
    let circle = Circle { radius: 32 };
    println!("{}", circle.to_string());
}

pub fn for_parse() {
    let parsed: i32 = "5".parse().unwrap();
    //涡轮鱼语法
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
