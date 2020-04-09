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
