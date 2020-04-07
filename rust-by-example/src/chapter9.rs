pub fn fizzbuzz2() {
    fizzbuzz_to(100);
}

//一个返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

//一个"不"返回值的函数，实际上会返回一个单元类型()
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

//方法是依附于对象的函数，这些方法通过关键字self来访问对象中的数据和其他。方法在impl代码块中定义
struct Point {
    x: f64,
    y: f64,
}

//实现的代码块，Point的所有方法都在这里给出
impl Point {
    //静态方法，不需要被实例调用，一般用作构造器
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    //    另一个静态方法，需要两个参数
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    //这是一个实例方法
    //&self是self:&Self的语法糖
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        //    'abs'是一个f64类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    //    这个方法要求调用者是可变的
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

//Pair拥有资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    //    这个方法会消耗调用者的资源
    fn destory(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({},{})", first, second);
    }
}

pub fn methods() {
    let rectangle = Rectangle {
        //静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    //    实例方法通过点运算符来调用
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    //rectangle是不可变的，但是translate方法需要一个可变对象
    // rectangle.translate(1.0, 1.0);
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();
    // 报错，前面的destory已经消耗了pair
    // pair.destory();
}
