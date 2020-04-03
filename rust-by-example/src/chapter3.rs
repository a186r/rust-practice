//该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

//单元结构体
struct Nil;

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
    let _nil = Nil;

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
    let x = Operations::Add;
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
