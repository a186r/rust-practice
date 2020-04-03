//该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

use List::*;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

//单元结构体
// struct Nil;

//元组结构体（具名元组）
struct Pair(i32, f32);

//带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//结构体可以作为另一个结构体的字段
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

pub fn structs() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    //以Debug方式打印结构体
    println!("{:?}", peter);
    //实例化结构体
    let point: Point = Point { x: 1.0, y: 3.4 };
    println!("point:({}, {})", point.x, point.y);

    let new_point = Point { x: 3.2, y: 0.1 };
    println!("new_point:({}, {})", new_point.x, new_point.y);

    //    使用let解构来绑定point
    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        //结构体的实例化也是一个表达式
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };
    //    实例化一个单元结构体
    // let _nil = Nil;

    //    实例化一个元组结构体
    let pair = Pair(1, 0.1);

    //    访问元组结构体的字段
    println!("元组结构体包含:{}, {}", pair.0, pair.1);
    //    解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair包含:{}, {}", integer, decimal);
}

pub fn for_test() {
    let rectangle = Rectangle {
        p1: Point { x: 0.5, y: 0.5 },
        p2: Point { x: 0.6, y: 0.9 },
    };
    rect_area(rectangle);
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { p1, p2 } = rectangle;
    let area = (p2.y - p1.y).abs() * (p2.x - p1.x).abs();
    println!("长方形的面积为:{}", area);
    area
}

pub fn for_square() {
    let new_square = square(Point { x: 1.0, y: 2.0 }, 3.0);
    println!("新构建的长方形是: {:?}", new_square);
    println!("新长方形的面积是:{:?}", rect_area(new_square));
}

fn square(point: Point, len: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        p1: Point { x, y: y + len },
        p2: Point { x: x + len, y },
    }
}

enum WebEvent {
    //一个enum可以是单元结构体
    PageLoad,
    PageUnload,
    //    或者一个元组结构体
    KeyPress(char),
    Paste(String),
    //    或者一个普通的结构体
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        //从enum里解构出c
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

pub fn for_inspect() {
    let pressed = WebEvent::KeyPress('x');
    //to_owned从一个字符串切片中创建一个具有所有权的string
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 23, y: 66 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

//创建一个类型别名
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

pub fn for_type_alias() {
    let _x = Operations::Add;
}

//最常见的情况就是在impl块中使用self别名
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn for_use() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Soldier;

    match status {
        //没有使用完整路径，是因为上面显式的用了use
        Rich => println!("rich"),
        Poor => println!("poor"),
    }

    match work {
        Civilian => println!("civilian"),
        Soldier => println!("soldier"),
    }
}

//拥有隐式辨别值的enum
enum Number {
    Zero,
    One,
    Two,
}

//拥有显式辨别值的enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn for_c() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("r are #{:06X}", Color::Red as i32);
    println!("v are #{:06X}", Color::Blue as i32);
}

//enum的一个常见用法就是创建链表(linked-list)
enum List {
    //Cons 元组结构体，包含链表的一个元素和一个指向下一个节点的指针
    Cons(u32, Box<List>),
    //末节点，表明链表结束
    Nil,
}

//可以为enum定义方法
impl List {
    //    创建一个空的List实例
    fn new() -> List {
        Nil
    }

    //    处理一个List，在其头部插入新元素，并返回该List
    fn prepend(self, elem: u32) -> List {
        //    Cons同样为List类型
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        //    必须对self进行匹配，因为这个方法的行为取决于self的取值种类
        match *self {
            //    不能得到tail的所有权，因为self是借用的，因此使用一个对tail的借用
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    //    返回列表的字符串表示(该字符串是堆分配的)
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                //    format和print类似，但是返回的是一个堆分配的字符串，而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

pub fn create_linklist() {
    //    创建一个空链表
    let mut list = List::new();
    //    追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    //    显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

//全局变量是在在所有其他作用域之外声明的
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn for_const() {
    let n = 16;
    //    访问常量
    println!("this is {}", LANGUAGE);
    println!("the threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
