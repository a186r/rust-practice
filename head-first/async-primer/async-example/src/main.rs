use futures::executor::block_on;
use futures::join;
use std::io;

mod file;

fn main() -> io::Result<()> {
    println!("Program started");

    block_on(load_files());

    Ok(())
}

async fn load_files(){
    join!(load_files_1(), load_files_2());
}

async fn load_files_1() {
    let r1 = file::read_file("src/file1.txt").await;
    println!("file 1 size: {}", r1.unwrap().len());
}

async fn load_files_2() {
    let r2 = file::read_file("src/file2.txt").await;
    println!("file 2 size: {}", r2.unwrap().len());
}