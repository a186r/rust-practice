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

pub fn closures() {
    //    通过闭包和函数分别实现自增
    fn function(i: i32) -> i32 {
        i + 1
    }

    // 闭包是匿名的
    let colsure_annotated = |i: i32| -> i32 { i + 1 };
    let colsure_inferred = |i| i + 1;

    let i = 1;

    //    调用函数和闭包
    println!("function: {}", function(i));
    println!("1:{}", colsure_annotated(i));
    println!("2:{}", colsure_inferred(i));

    //    没有参数的闭包，返回一个i32类型
    let one = || 1;
    println!("3:{}", one());
}

pub fn capture() {
    use std::mem;
    let color = "green";
    //这个闭包会立即借用color，并将该借用和闭包本身存储到print变量中，color会一直保持被借用的状态知道print离开作用域
    let print = || println!("'color':{}", color);
    //调用闭包，闭包又借用color
    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();

    let reborrow = &mut count;
    println!("l:{}", reborrow);

    let movable = Box::new(3);
    // mem::drop要求T类型本身，所以闭包将会捕获变量的值，，在这种情况下，可复制类型将会复制给闭包，
    // 不可复制类型必须移动到闭包中
    // 因而，movable变量在这里立即移动到了闭包中
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };
    // consume消耗了该变量，所以该闭包只能调用一次
    consume();
    // consume();
}

//在｜之前使用move会强制闭包取得被捕获变量的所有权
pub fn l_move() {
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 借用检查不允许在变量被移动走之后继续使用它
    // println!("{} elements in vec", haystack.len());
}

//该函数将闭包作为参数并调用它
fn apply<F>(f: F)
where
    //闭包没有输入值和返回值
    F: FnOnce(),
{
    f();
}

//输入闭包，返回一个i32整型的函数
fn apply_to_3<F>(f: F) -> i32
where
    //闭包处理一个i32整型并返回一个i32整型
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn input_para() {
    use std::mem;
    let greeting = "hello";

    //    to_owned从借用的数据创建有所有权的数据
    let mut farewell = "goodbye".to_owned();
    //    捕获2个变量。通过引用捕获"greeting，通过值捕获farewell
    let diary = || {
        println!("I said {}", greeting);

        //    下文改变了farewell，因此要求闭包通过可变引用来捕获它
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        //    手动调用drop又要求闭包通过值获取farewell
        mem::drop(farewell);
    };

    //    以闭包作为参数
    apply(diary);

    //    闭包double满足apply_to_3的trait约束
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

fn apply2<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn for_apply2() {
    let x = 7;

    //捕获x到匿名类型中，并为它实现Fn
    //将闭包存储到print中
    let print = || println!("{}", x);
    apply2(print);
}

//定义一个函数，可以接受一个由Fn限定的范型F参数并调用它
fn call_me<F: Fn()>(f: F) {
    f()
}

//定义一个满足Fn约束的封装函数
fn function() {
    println!("I'm a function");
}

pub fn for_fn() {
    //    定义一个满足Fn约束的闭包
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

//按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    // 必须使用move关键字，表明所有的捕获都是通过值进行的，这是必须的，
    // 因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

pub fn for_fnm() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    let fn_mut2 = || println!("自己写的闭包:{}", "自己".to_owned());

    fn_plain();
    fn_mut();
    fn_mut2();
}

//标准库中使用闭包的例子
//Iterator::any
// pub trait Iterator2 {
//     //    被迭代的类型
//     type Item;
//
//     fn any<F> (&mut self, f: F) -> bool
//     where
//     F: FnMut(Self::Item) -> bool{}
// }

pub fn for_iter() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    //对vec的into_iter()举出i32,无需解构
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    //    对数组的iter举出&i32
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    //对数组的into_iter()通常举出&i32，需要解构
    println!("2 in array1: {}", array2.into_iter().any(|&x| x == 2));
}

// pub trait Iterator2 {
//     type Item;
//
//     fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
//     where
//         P: FnMut(&Self::Item) -> bool,
//     {
//     }
// }

pub fn for_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    //Find方法会把迭代器元素的引用传给闭包，迭代器元素自身是&i32类型，所以传给闭包的是&&i32类型
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2));
}

//Rust使用了高阶函数，HOF和惰性迭代器给RUst带来了函数式编程的风格
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn for_hof() {
    println!("Find the sum of all xxxx 1000");

    let upper = 1000;
    //    命令式的写法
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared
        }
    }
    println!("imperative style: {}", acc);

    //    函数式的写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("imperative style: {}", sum_of_squared_odd_numbers);
}

//发散函数绝不会返回
fn foo() -> ! {
    panic!("sdfasdf");
}

fn some_fn() {
    ()
}

// #[feature(never_type)]
pub fn for_diverging() {
    // let a: () = some_fn();
    // println!("这个方法会返回，你会看到这行输出");
    // let x: ! = panic!("这个调用不会返回");
    // println!("你看不到这行输出");

    fn sun_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0.. {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
}
