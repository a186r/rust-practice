// 在Rust中，所有类型都是默认栈分配的。
// 通过创建Box<T>，可以把值装箱boxed来使它在堆上分配。
// 箱子是一个智能指针，指向堆分配的T类型的值。
// 当箱子离开作用域时，其析构函数会被调用，内部的对象会被销毁，堆上分配的内存也会被释放。

// 被装箱的值可以使用*运算符进行引用，这会移除掉一层装箱

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 在堆上分配这个Point，并返回一个指向它的指针
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn for_box() {
    // 所有类型标注都不是必须的
    // let point: Point = origin();
    let point = origin();
    let rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // let boxed_rectangle = boxed_origin();

    // 函数的输出可以装箱
    let boxed_point: Box<Point> = Box::new(origin());

    // 两层装箱
    // let box_in_a_box = Box::new(Box::new(origin()));
    let box_in_a_box = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes in the stack",
        mem::size_of_val(&point)
    );

    println!(
        "Rectangle occupies {} bytes in the stack",
        mem::size_of_val(&rectangle)
    );

    // box的宽度就是指针宽度
    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&boxed_point)
    );

    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&boxed_rectangle)
    );

    println!(
        "Boxed poing occupies {} bytes in the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes in the stack",
        mem::size_of_val(&unboxed_point)
    );
}

pub fn for_vec() {
    // 迭代器可以被收集到vec中
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1, 2, 3, 4];
    println!("Initial vector: {:?}", xs);

    xs.push(5);
    println!("{:?}", xs);

    // 报错，不可变vec是不可增长的
    // collected_iterator.push(0);

    println!("vec size: {}", xs.len());

    println!("Second element: {}", xs[1]);

    // Pop移除vec的最后一个元素并将它返回
    println!("Pop last element: {:?}", xs.pop());

    for x in xs.iter() {
        println!(">> {}", x);
    }

    // 可以在迭代vec的同时，使用独立变量i来记录迭代次数
    for (i, x) in xs.iter().enumerate() {
        println!("p {} value {}", i, x);
    }

    // 使用iter_mut时，可变的vec在迭代的过程中，每个值都可以修改
    for x in xs.iter_mut() {
        *x *= 3
    }
    println!("Updated vec: {:?}", xs);
}

pub fn for_str() {
    let pangram: &'static str = "the quick brown fox jums over the lazy dog";
    println!("Pangram: {}", pangram);

    // 逆序迭代单词，这里并没有分配新的字符串
    for word in pangram.split_whitespace().rev() {
        println!(">> {}", word);
    }

    // 复制字符到一个vec，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的且可增长的String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    // 分配新内存并存储修改过的字符串
    let bob: String = alice.replace("dog", "cat");
    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}
