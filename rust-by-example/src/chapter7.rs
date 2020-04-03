//代码块也是表达式，所以在它们的赋值操作中可以作为右值
//需要注意的是如果代码块最后一条表达式结尾处有分号，返回值将变成()
pub fn expression() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };
    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
