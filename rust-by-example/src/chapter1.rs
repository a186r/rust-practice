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
}
