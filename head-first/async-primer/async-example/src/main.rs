mod file;
use futures::join;
use futures::executor::block_on;
use std::io;

//如果我们已经有了多线程，为什么我们需要异步，有2个主要优点：性能和简单性。
fn main() -> io::Result<()> {
    println!("Program started");
    block_on(load_files());
    Ok(())
}

async fn load_files() {
    join!(load_files1(), load_file2());
}

async fn load_files1() {
    let r1 = file::read_file("src/file1.txt").await;
    println!("file 1 size: {}", r1.unwrap().len());
}

async fn load_file2() {
    let r2 = file::read_file("src/file2.txt").await;
    println!("file 2 size: {}", r2.unwrap().len());
}