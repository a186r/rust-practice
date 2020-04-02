use std::fmt;
use std::fmt::{Display, Formatter};

pub fn compound_type() {
    let _logical: bool = true;

    let _a_float: f64 = 1.0;
    let _b_float = 2.0f64;
    let _an_integer = 2i32;

    let _default_float = 3.0;
    let _default_integer = 7;

    //    自动推断类型
    let mut inferred_type = 12;
    inferred_type = 234234242i64;

    //    变量的值可以改变
    let mut mutable = 12;
    mutable = 13;

    //    变量的类型不能改变
    // mutable = true;

    //    但是可以通过shadow掩蔽来覆盖前面的变量
    let _mutable = true;
}

pub fn literals() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("sadofsadfa {}", 1_000_000u32);
}

//元组可以充当函数的参数和返回值
pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    //    可以使用let把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn tuples() {
    //    包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, true, 'a');
    println!("1 {}", long_tuple.0);
    println!("2 {}", long_tuple.1);

    //    元组也可以充当元祖的元素
    let tuple_of_tuple = ((1u8, 2u16, 3u64), ("12", true), -2i16);
    //    元组可以打印
    println!("tuple_of_tuple {:?}", tuple_of_tuple);

    //    但是很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 5, 6, 1, 7, 8, 1, 2, 3, 4, 5);
    // println!("print long tuple: {:?}", too_long_tuple);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));

    //    创建单元素元组需要一个额外的逗号，为了和被括号包含的字面量作区分
    println!("1 element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //元组可以被解构，从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})\n({},{})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
