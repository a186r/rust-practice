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
