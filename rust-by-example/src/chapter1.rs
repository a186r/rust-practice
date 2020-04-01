use std::fmt;
use std::fmt::Formatter;

pub fn print() {
    println!("Hello, world!");
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");
    println!("{} days", 31u32);
    //    可以使用命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown for",
        verb = "jumps over"
    );

    //    可以在:后面指定特殊的格式
    println!("{} of {:b} people know binary, the other half don't", 2, 3);

    println!("My name is {0}, {1}, {0}", "bond", "James");

    //    创建一个包含单个i32的结构体
    #[allow(dead_code)]
    pub struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3));

    let pi = 3.141592653;
    // println!("{}", format!("{name:.*}", 3, name = pi));
    println!("{name:.*}", 3, name = pi);

    //不能使用fmt::Display或者fmt::Debug来打印
    #[allow(dead_code)] // 允许未使用的代码存在
    struct UnPrintable(i32);

    //derive属性会自动创建所需的实现，使这个struct可以使用fmt::Debug打印
    #[derive(Debug)]
    struct DebugPrintable(i32);
}

#[derive(Debug)]
pub struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn print_struct() {
    println!("{:?}", Structure(3));
    //使用derive的一个问题是不能控制输出的形式，如果我只想展示一个12怎么办
    println!("{:?}", Deep(Structure(12)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    //    美化打印
    println!("{:#?}", peter);
}

//定义一个元祖结构体
struct Structure1(i32);

impl fmt::Display for Structure1 {
    //这个trait要求`fmt`使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//带有2个数字的结构体
#[derive(Debug)]
struct MinMax(i64, i64);

//实现MinMax的Display
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.0, self.1)
    }
}

//为了比较，定义一个含有具名字段的结构体
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

//对Point2D实现Display
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn for_display() {
    let minmax = MinMax(0, 12);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range,
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("What does point2D look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compare points:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

// //有了？ 对于一个Vec实现fmt::Display就很简单了
// //定义一个包含Vec的结构体List
// struct List(Vec<i32>);
//
// impl fmt::Display for List {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         //    使用元祖的下标获取值，并创建一个vec的引用
//         let vec = &self.0;
//         write!(f, "[")?;
//
//         //    使用v对vec进行迭代，并用count记录迭代次数
//         for (count, v) in vec.iter().enumerate() {
//             if count != 0 {
//                 write!(f, ",")?;
//             }
//             write!(f, "{}:{}", count, v)?;
//         }
//
//         //    加上配对中括号，并返回一个fmt::Result的值
//         write!(f, "]")
//     }
// }
//
// pub fn print_list() {
//     let v = List(vec![1, 2, 3, 4, 5]);
//     println!("{}", v);
// }

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //    首先获取值
        let vec = &self.0;
        //创建一个vec
        write!(f, "[")?;
        //    迭代并且记录迭代次数
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?; // 这一步是打印分隔符
            }
            write!(f, "{}:{}", count, v)?; // 这一步是打印  下标:值
        }

        write!(f, "]")
    }
}

pub fn print_list() {
    let vec = List(vec![1, 2, 3, 4, 5]);
    println!("{}", vec);
}
