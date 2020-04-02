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
struct Point {
    x: f32,
    y: f32,
}

//结构体可以作为另一个结构体的字段
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
