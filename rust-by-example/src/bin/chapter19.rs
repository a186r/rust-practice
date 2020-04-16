// 在Rust中，所有类型都是默认栈分配的。
// 通过创建Box<T>，可以把值装箱boxed来使它在堆上分配。
// 箱子是一个智能指针，指向堆分配的T类型的值。
// 当箱子离开作用域时，其析构函数会被调用，内部的对象会被销毁，堆上分配的内存也会被释放。

// 被装箱的值可以使用*运算符进行引用，这会移除掉一层装箱

use crate::bin::chapter19::checked::{div, ln, sqrt, MathError, MathResult};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个Point，并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn for_box() {
    // 所有类型标注都不是必须的
    // let point: Point = origin();
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // let boxed_rectangle = boxed_origin();

    // 函数的输出可以装箱
    let boxed_point: Box<Point> = Box::new(origin());

    // 两层装箱
    // let box_in_a_box = Box::new(Box::new(origin()));
    let box_in_a_box = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes in the stack",
        mem::size_of_val(&point)
    );

    println!(
        "Rectangle occupies {} bytes in the stack",
        mem::size_of_val(&rectangle)
    );

    // box的宽度就是指针宽度
    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&boxed_point)
    );

    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&boxed_rectangle)
    );

    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes in the stack",
        mem::size_of_val(&unboxed_point)
    );
}

pub fn for_vec() {
    // 迭代器可以被收集到vec中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1, 2, 3, 4];
    println!("Initial vector: {:?}", xs);

    xs.push(5);
    println!("{:?}", xs);

    // 报错，不可变vec是不可增长的
    // collected_iterator.push(0);

    println!("vec size: {}", xs.len());

    println!("Second element: {}", xs[1]);

    // Pop移除vec的最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());

    for x in xs.iter() {
        println!(">> {}", x);
    }

    // 可以在迭代vec的同时，使用独立变量i来记录迭代次数
    for (i, x) in xs.iter().enumerate() {
        println!("p {} value {}", i, x);
    }

    // 使用iter_mut时，可变的vec在迭代的过程中，每个值都可以修改
    for x in xs.iter_mut() {
        *x *= 3
    }
    println!("Updated vec: {:?}", xs);
}

pub fn for_str() {
    let pangram: &'static str = "the quick brown fox jums over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并没有分配新的字符串
    for word in pangram.split_whitespace().rev() {
        println!(">> {}", word);
    }

    // 复制字符到一个vec，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的且可增长的String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

pub fn for_str_v2() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
}

// 有时候想捕捉到程序某部分的失败信息，而不是调用panic!;这可使用Option枚举类型来实现

// 不会panic的整数除法
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        // 结果Result被包装到Some取值中
        Some(dividend / divisor)
    }
}

// 此函数处理可能失败的除法
fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
    }
}

pub fn for_option() {
    try_division(4, 2);
    try_division(1, 0);

    // 绑定None到一个变量需要类型标注
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // 解包Some将取出被包装的值
    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );

    // 解包None将会引发panic!
    // println!("{:?} unwraps to {:?}", none, none.unwrap());
}

mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // 此操作将会失败，与其让程序崩溃，不如将失败的原因包装在Err中返回
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    // 中间函数
    // ?问号运算符
    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;
        sqrt(ln)
    }

    pub fn op_v2(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!(match why {
                MathError::NegativeLogarithm => "logarithm of negative number",
                MathError::NegativeSquareRoot => "square root of negative number",
                MathError::DivisionByZero => "division by zero",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}

fn op(x: f64, y: f64) -> f64 {
    // 三层的match金字塔
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

pub fn for_result() {
    println!("{}", op(100.0, 1.0));
}

pub fn for_qmark() {
    checked::op_v2(1.0, 10.0);
}

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

pub fn for_hash() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.insert("Deniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

// 为了实验HashMap中的struct，试着做一个非常简单的用户登录系统
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

pub fn for_altkey() {
    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "j.everyman",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");
}

pub fn for_hashset() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果值已经存在，那么insert将返回false
    // assert!(b.insert(4), "Value 4 is already in set B!");

    b.insert(5);

    println!("A: {:?}", a);
    println!("A: {:?}", b);

    // 乱序打印
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
}
