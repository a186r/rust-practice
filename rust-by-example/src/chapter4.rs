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

//变量绑定默认是不可变的，但是加上mut修饰语后变量就可以改变
pub fn mut_variable() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
}

pub fn scope_variable() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short:{}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    println!("outer long: {}", long_lived_binding);
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

//可以先声明变量绑定，然后才将它们初始化,但是这种做法很少用，因为这样可能导致使用未初始化的变量
pub fn declare_variable() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;
    another_binding = 1;
    println!("another binding: {}", another_binding);
}
