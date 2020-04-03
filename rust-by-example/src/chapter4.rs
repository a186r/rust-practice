//变量绑定
pub fn variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let uint = ();

    let copy_integer = an_integer;

    println!("An integer: {:?}", copy_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", uint);

    //    编译器会对未使用的变量绑定产生警告，可以给变量名加上下划线前缀来消除警告
    let _unused_variable = 3u32;
}
