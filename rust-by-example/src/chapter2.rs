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
