use std::fmt::Display;

// // struct A;
// //
// // struct Single(A);
// //
// // struct SingleGen<T>(T);
// //
// // pub fn for_generics() {
// //     let _s = Single(A);
// //
// //     let _char: SingleGen<char> = SingleGen('a');
// //
// //     //SingleGen的类型参数也可以隐式的指定
// //     let _t = SingleGen(A);
// //     let _i32 = SingleGen(5);
// //     let _char = SingleGen('b');
// // }
// struct A; // 具体类型A
// struct S(A); // 具体类型S
// struct SGen<T>(T); // 范型类型SGen
//
// fn reg_fn(_s: S) {}
//
// fn gen_spec_t(_s: SGen<A>) {}
//
// fn gen_spec_i32(_s: SGen<i32>) {}
// //这个函数是关于T的范型函数
// fn generic<T>(_s: SGen<T>) {}
//
// pub fn for_gen() {
//     //    使用非范型函数
//     reg_fn(S(A));
//     //隐式的指定类型参数A
//     gen_spec_t(SGen(A));
//     //隐式的指定类型参数i32
//     gen_spec_i32(SGen(12));
//
//     //为generic显式的指定类型参数char
//     generic::<char>(SGen('a'));
//     generic(SGen('b'));
// }
//
//和函数类似，impl块想要实现范型，也需要很仔细
struct S;
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {}

//T必须在类型之前写出来，以使T代表范型
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

//Val的impl
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

//GenVal的impl
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

pub fn for_genval1() {
    let x = Val { val: 3.2 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}

//不可复制的类型
struct Empty;
struct Null;

//T的范型trait
trait DoubleDrop<T> {
    //    定义一个调用者的方法，接受一个额外的参数T，但不对它做任何事
    fn double_drop(self, _: T);
}

impl<U, T> DoubleDrop<T> for U {
    //此方法获得2个传入参数的所有权，并释放它们
    fn double_drop(self, _: T) {}
}

pub fn for_gen_trait() {
    let empty = Empty;
    let null = Null;
    //    释放empty和null
    empty.double_drop(null);

    // empty;
    // null;
}

//定义一个函数，接受一个类型为范型T的参数
//其中T必须实现Display trait
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct SS<T: Display>(T);

//约束的另一个作用是范型的实例可以访问作为约束的trait的方法
//这个trait用来实现打印标记
use std::fmt::Debug;

//trait就是接口
//实现trait就是实现接口
trait HasArea {
    fn area(&self) -> f64;
}

//为Rectangle实现HasArea trait
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn for_bounds() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}

//某些情况下也可以使用where分句来形成约束，这拥有更好的表现力

//约束的工作机制会产生这样的效果：即使一个trait不包含任何功能，你仍然可以使用它作为约束，标准库重的Eq和Ord就是这样的trait
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(t: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(t: &T) -> &'static str {
    "blue"
}

pub fn for_testcase() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _tur_key = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}

//多重约束可以用+连接，和平常一样，类型之间使用，隔开
//多重约束用加号连接
fn compare_print<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

//类型之间用逗号隔开
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

pub fn for_multi() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_print(&string);
    compare_types(&array, &vec);
}

//where分句，约束也可以使用where分句来表达
// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// //使用where从句来表达约束
// impl<A, D> MyTrait<A, D> for YourType
// where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF,
// {
// }

//当使用where从句比正常语法更有表现力时，本例中的impl如果不用where从句，就无法直接表达
trait PrintInOption {
    fn println_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn println_in_option(self) {
        println!("{:?}", Some(self));
    }
}

pub fn for_where() {
    let vec = vec![1, 2, 3];
    vec.println_in_option();
}

//newtype惯用法，能保证在编译时，提供给程序的都是正确的类型
struct Years(i64);
struct Days(i64);

//为struct实现
impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

pub fn for_new_types() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}

// struct Container(i32, i32);
//
// //这个trait检查给定的2个项是否存储与容器中
// //并且能够获得容器的第一个或最后一个值
// trait Contains<A, B> {
//     fn contains(&self, _: &A, _: &B) -> bool; // 显式的要求A和B
//     fn first(&self) -> i32; // 未显式的要求A和B
//     fn last(&self) -> i32;
// }
//
// impl Contains<i32, i32> for Container {
//     //如果存储的数字和给定的相等则为真
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         &self.0 == number_1 && &self.1 == number_2
//     }
//
//     fn first(&self) -> i32 {
//         self.0
//     }
//
//     fn last(&self) -> i32 {
//         self.1
//     }
// }
//
// //容器C就包含了A和B类型，鉴于此，必须指出A和B显得很麻烦
// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Contains<A, B>,
// {
//     container.last() - container.first()
// }
//
// pub fn for_assoc() {
//     let number_1 = 3;
//     let number_2 = 10;
//
//     let container = Container(number_1, number_2);
//
//     println!(
//         "Does container contain {} and {}: {}",
//         &number_1,
//         &number_2,
//         container.contains(&number_1, &number_2)
//     );
//
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());
//     println!("The difference is: {}", difference(&container));
// }

struct Container(i32, i32);
//通过把容器内部的类型放到trait中作为输出类型，使用关联类型增加了代码的可读性，这样的trait的定义语法如下
trait Contains {
    type A;
    type B;

    //    这种语法能够范型的表示这些新类型
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;

    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// //使用关联类型
// fn difference<C:Contains>(container: &C) -> i32{
//
// }

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        &self.0 == number_1 && &self.1 == number_2
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

//约束类型C必须实现Contains trait
fn difference<C: Contains>(c: &C) -> i32 {
    c.last() - c.first()
}

pub fn for_types() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {} :{}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First: {}", container.first());
    println!("Last: {}", container.last());
    println!("The difference is: {}", difference(&container));
}
