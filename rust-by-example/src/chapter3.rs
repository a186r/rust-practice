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
    let square = square(Point { x: 1.0, y: 2.0 }, 3.0);
    println!("新构建的长方形是: {:?}", square);

    println!("新长方形的面积是:{:?}", rect_area(square));
}

fn square(point: Point, len: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        p1: Point { x, y: y + len },
        p2: Point { x: x + len, y },
    }
}
