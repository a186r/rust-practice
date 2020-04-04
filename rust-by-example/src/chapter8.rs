#![allow(unreachable_code)]
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
