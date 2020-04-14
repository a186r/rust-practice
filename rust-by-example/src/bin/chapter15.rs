use crate::bin::chapter15::Either::Num;
use std::fmt::Debug;

//Rust强制实行资源获取即初始化，所以任何对象在离开作用域时，它的析构函数就被调用，然后它占有的资源就被释放
fn create_box() {
    //    在堆上分配一个整型数据
    let _box1 = Box::new(3i32);
    //    在这里_box1被销毁，资源得到释放，，在离开作用域时，这里就将要离开作用域了
}

fn t() {
    //    在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    //    嵌套作用域

    {
        let _box3 = Box::new(4i32);
    }

    //    创建一大堆Box，完全不需要手动释放内存
    for _ in 0..1_000 {
        create_box();
    }
    //    _box2在这里被销毁，内存得到释放，，在离开作用域时，这里就将要离开作用域了
}

// //析构函数
// struct ToDrop;
//
// impl Drop for ToDrop {
//     fn drop(&mut self) {
//         println!("ToDrop is being dropped");
//     }
// }
//
// fn main() {
//     let x = ToDrop;
//     println!("Made a ToDrop!");
//     //    当main函数的变量离开作用域，自定义的析构函数就会被调用
// }

//因为变量要负责释放它们拥有的资源，所以资源只能拥有一个所有者，这也防止了资源的重复释放，注意，并非所有变量都拥有资源
//在进行赋值或者通过值来传递函数参数的时候，资源的所有权会发生转移，在Rust里，这杯成为Move，移动
//在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针的产生

//此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    //    c被销毁并且内存得到释放
}

pub fn for_move() {
    //栈分配的整型
    let x = 5u32;
    //将x复制到y——不存在资源的移动
    let y = x;

    println!("x is {} and y is {}", x, y);

    //    a是一个指向堆分配的整数的指针
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    // 移动a到b
    // 把a的指针地址复制到b，现在两者都指向同一个堆分配的数据，但是现在是b拥有它
    let b = a;
    // a不能访问数据
    // println!("a contains: {}", a);

    //此函数从b中取得堆分配的内存的所有权
    destroy_box(b);
    // println!("b is {}", b);
}

//可变性，当所有权转移时，数据的可变性可能发生改变
pub fn for_mut() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    //可变性错误
    // *immutable_box = 4;

    //移动box，改变所有权(和可变性)
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}

//多数情况下，我们更希望能访问数据，同时不取得其所有权，为了实现这点，Rust使用了借用borrowing机制
//对象可以通过引用&T来传递，从而取代通过值T来传递

//此函数取得一个box的所有权并性销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destorying box that contains {}", boxed_i32);
}

//此函数借用了一个i32类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

pub fn for_borrow() {
    //    创建一个装箱的i32类型，以及一个存在栈中的i32类型
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

//此函数接受一个对Book类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

//此函数接受一个对可变的Book类型的引用，它将年份改为2014年
fn new_edition(book: &mut Book) {
    book.year = 2020;
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

pub fn for_mut1() {
    let immutabook = Book {
        author: "Douagsadflkj",
        title: "dsafagasgsd",
        year: 1967,
    };

    //    创建一个immutabook的可变拷贝
    let mut mutabook = immutabook;

    //    不可变地借用一个不可变对象
    borrow_book(&immutabook);
    //    不可变地借用一个可变对象
    borrow_book(&mutabook);

    //    可变地借用一个可变对象
    new_edition(&mut mutabook);
    //    可变地借用给一个不可变对象——错误
    // new_edition(&immutabook);
}

//当数据被不可变地借用时，它还会冻结freeze，已冻结的数据无法通过原始对象来修改，直到对这些数据的所有引用离开作用域为止
pub fn for_freeze() {
    let mut _mutable_integer = 7i32;

    {
        let large_integer = &_mutable_integer;
        // _mutable_integer = 50; // 报错，在被作用域被冻结
        println!("Immutably borrowed {}", large_integer);
    }

    //    正常运行，在这个作用域没有冻结
    _mutable_integer = 3i32;
}

//数据可以进行多次不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用，另一方面，在同一时刻内只允许有一个可变借用。
//只有在可变引用离开作用域之后，原始数据才可再次被借用
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn for_alias() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;
        //    通过引用和原始所有者来访问数据
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y, borrowed_point.z
        );

        // 不能可变的借用point，因为它现在有不可变的借用，出作用域才可以可变的借用，因为数据在不可变的借用时，已经被冻结了
        // let mutable_borrow = &mut point;
        // 此处再次使用被借用的值
        println!(
            "Point has corrdinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );
        // 到这里，上面的不可变借用才走出作用域，这行之后，数据才被解冻，可以声明和使用不可变借用
    }

    {
        let mut mutable_borrow = &mut point;

        //    通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错，不能打印，不能不可变的借用point，因为现在它有可变借用了
        // let y = &point.y;

        // 报错，不能打印，因为println会创建一个不可变引用
        // println!("Point Z coordinate is {}", point.z);

        // 可以工作，可变引用可以作为不可变的传给println
        println!(
            "Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );

        //    可变引用离开作用域
    }
    // 现在又可以不可变地借用point了
    let borrowed_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z
    );
}

//通过let绑定来进行模式匹配或者解构时，ref关键字可以用来创建结构体/元组的字段的引用。
#[derive(Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

pub fn for_ref() {
    let c = 'Q';

    // 赋值语句左边的ref关键字等价于右边的 & 符号
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point2 = Point2 { x: 0, y: 0 };

    // 在解构一个结构体时，ref同样有效
    let _copy_of_x = {
        //解构结构体
        let Point2 {
            x: ref ref_to_x,
            y: _,
        } = point2;
        // 返回一个point的x字段的拷贝
        *ref_to_x
    };

    let mut mutable_point2 = point2;

    {
        let Point2 {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point2;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point2.x, point2.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point2.x, mutable_point2.y
    );

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构mutable_tuple来改变last的值
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

// 生命周期
pub fn for_lifetime() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1 is {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2 is {}", borrow2);
    }
}

// 函数接受2个i32的引用，它们有不同的生命周期'a和'b
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数的函数，不过又一个生命周期参数'a
pub fn failed_borrow<'a>() {
    let _x = 12;
    // 报错，生命周期比y短，短生命周期不能强制转换成长生命周期
    // let y: &'a i32 = &_x;
}

pub fn for_explicit() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}

// 一个拥有生命周期'a的输入引用，其中'a的存活时间至少与函数的一样长
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

// 可变引用同样也可能拥有生命周期
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

// 返回传递进来的引用也是可行的,但必须返回正确的生命周期
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

pub fn for_fn() {
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

//方法的标注和函数类似
struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

pub fn for_methods() {
    let mut owner = Owner(19);
    owner.add_one();
    owner.print();
}

// 在结构体中标注生命周期也和函数类似
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

pub fn for_struct() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is not borrowed in {:?}", number);

    // let b: Borrowed2 = Default::default();
    //前面已经声明b的类型了，所以直接使用Default::default()
    let b: Borrowed2 = Default::default();
    println!("b is {:?}", b);
}

//trait方法中生命周期的标注基本上与函数类似，impl也可能有生命周期的标注
#[derive(Debug)]
struct Borrowed2<'a> {
    x: &'a i32,
}

//给impl标注生命周期
impl<'a> Default for Borrowed2<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

// Ref包含一个指向泛型类型T的引用，其中T拥有一个未知的生命周期'a
// T拥有生命周期限制，T中的任何引用都必须比'a活得更长
// 另外，Ref的生命周期也不能超出'a
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// 一个泛型函数，使用Debug trait来打印内容
fn print<T>(t: T)
where
    T: Debug,
{
    println!("print: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
    T: Debug + 'a,
{
    println!("print_ref: t is {:?}", t);
}

pub fn for_bounds() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}

// 一个较长的生命周期可以强制转换成一个较短的生命周期，使它在一个通常情况下不能工作的作用域内也能正常工作。
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

//<'a: 'b, 'b> 读作'a至少和'b一样长
//接受一个&'a i32类型并返回一个&'b i32类型，强制转换得到结果
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

pub fn for_coercion() {
    let first = 2;

    {
        let second = 3;
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

//'static生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中存在，'static生命周期也可被强制转换成一个更短的生命周期
// 产生一个拥有'static生命周期的常量
static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn for_static_lt() {
    {
        // 产生一个string字面量并打印
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        let corece_static = coerce_static(&lifetime_num);
        println!("coerced_ static: {}", corece_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: {}", x);
}

fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

pub fn for_elision() {
    let x = 3;

    elided_input(&x);
    elided_pass(&x);

    println!("elided_pass: {}", elided_pass(&x));
    println!("elided_input: {}", annotated_pass(&x));
}
