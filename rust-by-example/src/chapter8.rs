#![allow(unreachable_code)]
#[allow(dead_code)]
fn fi_else() {
    let n = 5;
    if n < 0 {
        println!("xxx");
    } else if n > 0 {
        println!("xxxx");
    } else {
        println!("0");
    }

    let big_n = if n < 10 && n > -10 {
        println!("xxxx");
        10 * n
    } else {
        println!("xxxxxx");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

pub fn for_loop() {
    let mut count = 0u32;
    println!("let's count +++++");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue; // 跳过这次迭代的剩下内容，也就是这次迭代，后面的代码不会再继续执行了，直接开始下一轮迭代
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, enough");
            break; // 退出loop循环
        }
    }
}

pub fn for_nested() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            break 'outer;
            break 'inner;
        }
        println!("this point will never be reached");
    }

    println!("exited the outer loop");
}

//从loop循环中返回
//loop有个用途是尝试一个操作知道成功为止，若操作返回一个值，则可能需要将其传递给代码的其余部分
//将该值放在break之后，它就会被loop表达式返回
pub fn return_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn for_while() {
    let mut n = 1;
    while n < 101 {
        println!("{}", n);
        n += 1;
    }
}

pub fn for_in() {
    // for n in 1..101 {
    //     println!("{}", n);
    // }

    // for n in 1..=100 {
    //     println!("{}", n);
    // }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    //在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用
    // for name in names.iter() {
    //     match name {
    //         &"Ferris" => println!("sadfasdagas"),
    //         _ => println!("NNNN"),
    //     }
    // }
    //会消耗集合，集合中的数据本身会被提供，一旦集合被消耗了，之后就再无法使用了，因为它已经在循环中被move了
    // for name in names.into_iter() {
    //     match name {
    //         "Ferris" => println!("SDSDFSF"),
    //         _ => println!("sdasfs"),
    //     }
    // }

    //可变地借用集合中的每个元素，从而允许集合被就地修改
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "there is a us",
            _ => "hello",
        }
    }
    println!("names: {:?}", names);
}

pub fn for_match() {
    let number = 13;

    println!("Tell me about {}", number);

    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 => println!("This is a prime!"),
        //匹配一个区间范围
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    //    match也是一个表达式
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

//元组可以在match中解构
pub fn for_destruct() {
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        //    解构出第二个值
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("不匹配"),
    }
}

//解构枚举
enum Color {
    //这三个值仅由他们的名字来指定
    Red,
    Blue,
    Green,
    //这些则把u32元组赋予不同名字，以色彩模型命名
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn for_enum() {
    let color = Color::HSL(122, 17, 40);

    println!("What color is it");

    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("r:{}, g:{}, b:{}", r, g, b),
        Color::HSV(h, s, v) => println!("h:{}, s:{}, v:{}", h, s, v),
        Color::HSL(h, s, l) => println!("h:{}, s:{}, l:{}", h, s, l),
        Color::CMY(c, m, y) => println!("c:{}, m:{}, y:{}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("c:{}, m:{}, y:{}, k:{}", c, m, y, k),
        //    不需要其他分支，因为所有的情形都已被覆盖
    }
}

//解引用使用*
//解构使用&、ref、和ref mut
pub fn for_refre() {
    let reference = &4;

    match reference {
        &vcl => println!("Got a value via destructuring: {:?}", vcl),
        //下面会报错，因为不能对整数解引用
        // &vcl => println!("Got a value via destructuring: {:?}", *vcl),
    }

    //    如果不想用&，需要在匹配前解引用
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    //下面这个不是引用，因为右边不是引用
    let _not_a_reference = 3;

    //    rust提供了ref，它更改了赋值行为，从而可以对具体值创建引用
    //    下面这行将得到一个引用
    let ref _is_a_reference = 3;

    //    相应的，定义两个非引用的变量，通过ref和ref mut仍可以取其引用
    let value = 5;
    let mut mut_value = 6;

    match value {
        // ref r => println!("Got a reference to a value: {:?}", r),
        ref r => println!("Got a reference to a value: {:?}", *r),
    }

    match mut_value {
        ref mut r => println!("Got a reference to a value: {:?}", *r),
    }

    match mut_value {
        ref mut m => {
            //    已经获得了mut_value的引用，先要解引用，才能改变它的值
            *m += 10;
            println!("We added 10. mut_value is : {:?}", m);
        }
    }
}

pub fn for_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    //    解构结构体的成员
    let foo = Foo { x: (1, 2), y: 14 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    //    可以解构结构i并重命名变量, 成员顺序并不重要
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j:{:?}", i, j);
    //    也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);
}

// 守卫
//match可以加上守卫来过滤分支
pub fn for_guard() {
    let pair = (-2, 2);

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        //    if条件部分是一个守卫
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 0 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

//绑定
//在match中，若间接的访问一个变量，则不经过重新绑定就无法在分支中使用它，match提供了@符号来绑定到变量名称
fn age() -> u32 {
    22
}

pub fn for_bind() {
    println!("Tell me type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1..=12 => println!("child of age:{:?}", n),
        n @ 13..=19 => println!("teen of age:{:?}", n),
        //    不符合上面的范围，返回结果
        a => println!("{:?}", a),
        //也可以写成这样
        // a @ _ => println!("{:?}", a),
    }
}

//也可以使用绑定来解构enum变体，例如Option
fn some_number() -> Option<u32> {
    // Some(55)
    None
}

pub fn for_option() {
    match some_number() {
        Some(n @ 42) => println!("The answer: {}!", n),
        Some(n) => println!("Not interesting ...{}", n),
        //None的情况
        _ => (),
    }
}

//在某些环境下，用match匹配枚举类型并不优雅，例如
fn feel_not_good() {
    let optional = Some(7);

    match optional {
        Some(i) => println!("this is a really long string an '{:?}'", i),
        _ => {}
    }
}

//使用if let在这样的场合要简洁的多，并且允许指明多种失败情况下的选项
pub fn feel_good() {
    //    全部都是Option<i32>类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    //若let将number解构成Some(i)，则执行语句块{}
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    //    如果要指明失败的类型，就使用else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        //解构失败
        println!("Err");
    }

    //    提供另一种失败情况下的条件
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("没有匹配到一个数字哦");
    } else {
        //条件的值为false，于是以下是默认的分支
        println!("我不喜欢letters，来到emoticon吧");
    }
}

//使用if let 匹配任何枚举值
#[derive(Debug)]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn if_let_match_enum() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(12);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Baz = b {
        println!("b is foobaz");
    }

    if let Foo::Qux(i) = c {
        println!("c is : {}", i);
    }
}

//枚举未注明#[derive(PartialEq)]，通常情况下if Foo2::Bar == a会出错，因为此类枚举的实例不具有可比性
//但是if let是可行的
enum Foo2 {
    Bar,
}

pub fn fix_enum() {
    let a = Foo2::Bar;

    //    变量匹配Foo2::Bar
    // if Foo2::Bar == a {
    //     println!("a is foobar")
    // }
    if let Foo2::Bar = a {
        println!("a is foobar");
    } else {
        println!("a is not foobar");
    }
}

//和if let 类似，while let也可以把别扭的match改写得好看一些
pub fn while_let() {
    let mut optional = Some(0);
    //重复运行这个测试
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("i is {:?}, Try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }
}

//使用while let可以使得代码更优雅
//当let将optional解构成Some(i)时，就执行语句块{},否则就break
pub fn much_beautiful() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("大于9");
            optional = None;
        } else {
            println!("i is {:?}, Try again. ", i);
            optional = Some(i + 1);
        }
    }
}
