//不显示类型转换产生的溢出警告
#![allow(overflowing_literals)]
//通过这个属性屏蔽警告
#![allow(non_camel_case_types)]

pub fn cast() {
    let decimal = 65.43266_f32;

    //    不提供隐式转换
    // let integer: u8 = decimal;

    //    可以显式转换
    let integer: u8 = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    //    1000已经在u16的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    //1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is: {}", 1000 as u8); //232

    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", -1i8 as u8); //255

    //对于正数，这就跟取模一样
    println!("1000 mod 256 is: {}", 1000 % 256);

    println!("128 as a i16 is: {}", 128 as i16);
    println!("128 as a i8 is: {}", 128 as i8);
}

//类型推断
pub fn type_inference() {
    let elem = 5u8;
    //    创建一个空向量,即不定长的，可以增长的数组
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}

//别名
type NanoSecond = u64;
type Inch = u64;
type u64_t = u64;

pub fn alias() {
    let nanosecond: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    //    类型别名并不呢个提供额外的类型安全，因为别名并不是新的类型
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanosecond,
        inches,
        nanosecond + inches
    );
}
