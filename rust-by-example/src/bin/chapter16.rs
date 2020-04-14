use std::ops;
use std::ops::Add;

// trait是对未知类型Self定义的方法集，该类型也可以访问同一个tait中定义的其他方法。
// 对任何数据类型都可以实现trait
struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // 静态方法签名，Self表示实现者类型
    fn new(name: &'static str) -> Self;

    // 实例方法签名，这些方法将返回一个字符串
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait可以提供默认的方法定义
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name);
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// 对Sheep实现Animal trait
impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    // 默认trait方法可以重载
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn for_trait() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

// 可以比较的元组结构体
#[derive(PartialOrd, PartialEq)]
struct Centimeters(f64);

// 可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        // 解构
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// 不带附加属性的元组结构体
struct Seconds(i32);

pub fn for_derive() {
    let _one_second = Seconds(1);

    let foot = Inches(12);
    println!("One foot equals: {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> Self::Output {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> Self::Output {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

pub fn for_ops() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

pub fn for_drop() {
    let _a = Droppable { name: "a" };

    // A
    {
        let _b = Droppable { name: "b" };
        // B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");

    // 变量可以手动使用drop来销毁
    drop(_a);
    println!("end of the main function");

    // _a不会在这里再次销毁，因为它已经被手动销毁
}

struct Fibonaci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonaci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;
        //    斐波那契数列不存在终点，那么Iterator将不可能返回None，而总是返回Some。
        Some(self.curr)
    }
}

// 返回一个斐波那契数列生成器
fn fibonacci() -> Fibonaci {
    Fibonaci { curr: 1, next: 1 }
}

pub fn for_iter() {
    // 0..3是一个Iterator，会产生0、1、2
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");

    // take(n)方法提取Iterator的前n项
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // skip(n)移除前n项，从而缩短了Iterator
    // 获取前8项？
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    // iter方法对数组产生一个Iterator
    for i in array.iter() {
        println!(">> {}", i);
    }
}

#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

pub fn for_clone() {
    let nil = Nil;
    // 复制Nil
    let copied_nil = Nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    //将pair绑定到moved_pair,移动了资源
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
    // println!("original: {:?}", pair); // 报错，pair已经失去了它的资源

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    println!("clone: {:?}", cloned_pair);
}
