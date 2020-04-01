pub fn print() {
    println!("Hello, world!");
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");
    println!("{} days", 31u32);
    //    可以使用命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown for",
        verb = "jumps over"
    );

    //    可以在:后面指定特殊的格式
    println!("{} of {:b} people know binary, the other half don't", 2, 3);

    println!("My name is {0}, {1}, {0}", "bond", "James");

    //    创建一个包含单个i32的结构体
    #[allow(dead_code)]
    pub struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3));

    let pi = 3.141592653;
    // println!("{}", format!("{name:.*}", 3, name = pi));
    println!("{name:.*}", 3, name = pi);

    //不能使用fmt::Display或者fmt::Debug来打印
    #[allow(dead_code)] // 允许未使用的代码存在
    struct UnPrintable(i32);

    //derive属性会自动创建所需的实现，使这个struct可以使用fmt::Debug打印
    #[derive(Debug)]
    struct DebugPrintable(i32);
}

#[derive(Debug)]
pub struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn print_struct() {
    println!("{:?}", Structure(3));
    //使用derive的一个问题是不能控制输出的形式，如果我只想展示一个12怎么办
    println!("{:?}", Deep(Structure(12)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    //    美化打印
    println!("{:#?}", peter);
}
